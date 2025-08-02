import json
import networkx as nx
import matplotlib.pyplot as plt

DATSET_PATH = "../dataset/"
GRAPH_PATH = DATSET_PATH + "graph.json"
SUBGRAPH_PATH = DATSET_PATH + "mst_graph.json"
GRAPH_VISUALIZATION_PATH = DATSET_PATH + "city_graph_visualization.png"
SUBGRAPH_VISUALIZATION_PATH = DATSET_PATH + "mlt_city_graph_visualization.png"


def load_json_graph(file_path: str):
    """Load graph data from a JSON file."""
    with open(file_path, "r") as f:
        data = json.load(f)
    return data


def build_graph(grah_path: str) -> nx.Graph:
    graph = load_json_graph(grah_path)

    adjacency_list = graph["adjacency_list"]
    cities = graph["cities"]

    G = nx.Graph()

    for city_name, city_info in cities.items():
        G.add_node(city_name, country=city_info["country"])

    # Add edges with weights (distances)
    for from_city, neighbors in adjacency_list.items():
        for to_city, distance in neighbors.items():
            # Since graph is undirected and edges are duplicated,
            # only add edge if not already added
            if G.has_edge(from_city, to_city):
                continue
            G.add_edge(from_city, to_city, weight=distance)

    return G


def build_graph_visualization(G: nx.Graph):
    plt.figure(figsize=(14, 10))

    pos = nx.kamada_kawai_layout(G)

    degrees = dict(G.degree())
    node_sizes = [max(50, deg * 50) for deg in degrees.values()]
    node_colors = list(degrees.values())

    nx.draw_networkx_nodes(
        G,
        pos,
        node_size=node_sizes,
        node_color=node_colors,
        cmap=plt.cm.plasma,
        alpha=0.9,
    )

    weights = [G[u][v]["weight"] for u, v in G.edges()]
    max_weight = max(weights)
    edge_widths = [0.5 + (weight / max_weight) * 2 for weight in weights]

    nx.draw_networkx_edges(G, pos, width=edge_widths, alpha=0.4, edge_color="gray")

    nx.draw_networkx_labels(G, pos, font_size=8, font_color="black")

    # Show distances (edge weights) on edges
    edge_labels = nx.get_edge_attributes(G, "weight")
    nx.draw_networkx_edge_labels(G, pos, edge_labels=edge_labels, font_size=7)

    plt.title("City Graph with Distances")
    plt.axis("off")
    plt.tight_layout()
    return plt


def save_graph_visualization(plt, filename: str):
    """Save the graph visualization to a file."""
    plt.savefig(filename, format="png", dpi=300)


if __name__ == "__main__":
    G = build_graph(GRAPH_PATH)
    plt = build_graph_visualization(G)
    save_graph_visualization(plt, GRAPH_VISUALIZATION_PATH)

    G_sub = build_graph(SUBGRAPH_PATH)
    plt_sub = build_graph_visualization(G_sub)
    save_graph_visualization(plt, SUBGRAPH_VISUALIZATION_PATH)
