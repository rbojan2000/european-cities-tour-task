<a name="readme-top"></a>
<br />
<div align="center">
  <h1 align="center">European Cities Tour Task Solution</h1>
</div>

<!-- TABLE OF CONTENTS -->
<details>

## Table of Contents

- [About The Project](#about-the-project)
- [Solution](#solution)
  - [Tasks](#tasks)
    - [Build Graph](#build-graph)
    - [Build MST](#build-mst)
    - [Find Best Path](#find-best-path)
      - [Strategies](#strategies)
        - [Brute Force (BF)](#brute-force-bf)
    - [Room for Improvement](#room-for-improvement)
- [How to Use](#how-to-use)
  - [Prerequisites](#prerequisites)
  - [Setup Instructions](#setup-instructions)
  - [Commands](#commands)
- [Evaluation](#evaluation)
  - [System Information](#system-information)
  - [Benchmark Summary](#benchmark-summary)

</details>

---

# About The Project

This project solves a **Traveling Salesman Problem (TSP)** variant over a graph of European cities using different algorithms such as graph construction, MST approximation, and brute-force pathfinding.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

# Solution

## Tasks

### Build Graph
- This task reads data from the <a href = "https://github.com/rbojan2000/european-cities-tour-task/blob/main/dataset/input.txt" > <i> dataset/input.txt </i> </a> file and constructs an <b> undirected weighted graph </b>. Each node in the graph represents a European city (identified by its name), and edges represent distances between cities. The graph is implemented as an adjacency list in the following format:

```sh
Map<String, Map<String, uint>>
```

Additionally, a `cities` lookup dictionary is created to map each city to its corresponding country:

```sh
HashMap<String, String> // {city: country}
```

Once constructed, the graph is serialized to JSON and saved to <a href = "https://github.com/rbojan2000/european-cities-tour-task/blob/main/dataset/graph.json"> <i> dataset/graph.json </i> </a>, allowing other tasks to load and use it directly without rebuilding.

> [!NOTE]
> The graph is already built and serialized as dataset/graph.json.

An initial visualization of the graph is available as <a href ="https://github.com/rbojan2000/european-cities-tour-task/blob/main/dataset/city_graph_visualization.png" > <i> city_graph_visualization.png </i> </a> .
[![](/dataset/city_graph_visualization.png)](/dataset/city_graph_visualization.png)


### Build MST
This task generates a Minimum Spanning Tree (MST) from the existing graph, which can be used as an approximation for solving the Traveling Salesman Problem (TSP) more efficiently.

It reads the graph structure from the serialized <a href = "https://github.com/rbojan2000/european-cities-tour-task/blob/main/dataset/graph.json"> <i> dataset/graph.json </i> </a> file, constructs the MST using a suitable algorithm (Kruskal's), and saves the resulting subgraph to a new file <a href = "https://github.com/rbojan2000/european-cities-tour-task/blob/main/dataset/mst_graph.json"> <i> dataset/mst_graph.json </i> </a>.

> [!NOTE]
> The MST graph is already built and serialized as dataset/mst_graph.json.

An initial visualization of the mst subgraph is available as <a href ="https://github.com/rbojan2000/european-cities-tour-task/blob/main/dataset/mlt_city_graph_visualization.png" > <i> mst_city_graph_visualization.png </i> </a> .
[![](/dataset/mlt_city_graph_visualization.png)](/dataset/mlt_city_graph_visualization.png)


### Find Best Path
This task finds the shortest possible route that visits all the input cities exactly once using the MST-based graph. It's designed to solve a variant of the Traveling Salesman Problem (TSP).

#### Strategies

##### Brute Force (BF)
The brute-force approach evaluates every possible permutation of the input cities to determine the path with the smallest total distance.

Steps:
1. Generate permutations
    - All possible orderings of the input cities are generated.
    - Number of permutations = n! (where n is the number of cities)

2. Build distance matrix
    - For each city pair, find the shortest distance (via BFS) based on the MST graph.

3. Evaluate all paths
    - Calculate the total distance for each permutation and select the one with the lowest score.

Example:
Input cities: `Paris`, `Barcelona`, `Madrid`


1. Generated permutations (3! = 6):
```yaml
[Paris, Barcelona, Madrid]
[Paris, Madrid, Barcelona]
[Barcelona, Paris, Madrid]
[Barcelona, Madrid, Paris]
[Madrid, Paris, Barcelona]
[Madrid, Barcelona, Paris]
```

2. distance matrix:

```yaml
           Barcelona     Paris    Madrid
 Barcelona         0       850       620
     Paris       850         0      1050
    Madrid       620      1050         0
```

3. Best path (lowest score):

```yaml
Path: ["Paris", "Barcelona", "Madrid"] with score: 1470
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Room for Improvement
1. Avoid generating all permutations at once
  - Instead of calculating all permutations upfront (which is computationally expensive and memory-intensive), generate permutations on the fly, based on their index.
  - How it works:
    - There are n! total permutations.
    - For any number i in the range 0..n!, i-th can be deriveed using factorial number system (a.k.a. Lehmer code).
  - Steps:
    - Start with a list of city indices: [0, 1, 2, ..., n-1]
    - Convert the number i into factorial base (factoradic representation).
    - Use that to determine the permutation by selecting indices accordingly.
  - Example:
  ```yaml
  let cities = vec!["London", "Paris", "Lyon"];
  let i = 5; // index of permutation we want
  ```
  - Total permutations: 3! = 6
  - Factoradic of 5 (with padding for length 3) = [2, 1, 0]
  - Apply this to [0, 1, 2] → gives permutation indices: [2, 1, 0]
  - Resulting permutation: ["Lyon", "Paris", "London"]

# How to Use

## Prerequisites

Make sure the following tools are installed:

- `rustc 1.88.0`
- `cargo 1.88.0`

## Setup Instructions

1. Clone the repository:
    ```sh
    git clone https://github.com/rbojan2000/european-cities-tour-task.git
    ```

2. Navigate into the project directory and build the executable:
    ```sh
    cd european-cities-tour-task/ect
    cargo build --release
    ```

3. Run the app:
    ```sh
    ./target/release/ect
    ```

### Commands
Below are the examples of how to run the task solution:

#### Examples
1. Run serial implementation:
    ```sh
    ./target/release/ect --task find-best-path --algorithm serial
    ```

2. Run parallel implementation with 2 threads:
    ```sh
    ./target/release/ect --task find-best-path --algorithm parallel --num-threads 2
    ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

# Evaluation

## System Information
All tests were executed on the following local development machine:\

- <b> CPU </b>: AMD Ryzen 5 5500U with Radeon Graphics
  - 6 cores / 12 threads
  - Base frequency: 2.1 GHz, Max turbo: ~4.05 GHz
  - Architecture: x86_64
- <b> RAM </b>:
  - Total: 18 GiB
- <b> Operating System </b>: Fedora Linux

## Benchmark Summary
Below are the measured execution times for finding the best paths in four typical cases:

### Input Case 1 — 8 Cities

```yaml
[Barcelona, Paris, Madrid, London, Prague, Frankfurt, Zurich, Lyon]
```

### Input Case 2 — 12 Cities

```yaml
[Barcelona, Paris, Madrid, London, Prague, Frankfurt, Zurich, Lyon, Amsterdam, Vienna, Rome, Milan]
```

### Input Case 3 — 16 Cities

```yaml
[Barcelona, Paris, Madrid, London, Prague, Frankfurt, Zurich, Lyon, Amsterdam, Vienna, Rome, Milan, Berlin, Geneva, Florence, Munich]
```

### Input Case 4 — 20 Cities

```yaml
[Barcelona, Paris, Madrid, London, Prague, Frankfurt, Zurich, Lyon, Amsterdam, Vienna, Rome, Milan, Berlin, Geneva, Florence, Munich, Budapest, Brussels, Zagreb, Venice]
```

| Input Case | Strategy | Mode   | Threads | Time (sec) | Best Path                                                                                       | Score |
|------|----------|--------|---------|-------------|--------------------------------------------------------------------------------------------------|--------|
| 1    | BF       | Serial | —       | 0.03568      | ["Madrid", "Barcelona", "Lyon", "Paris", "London", "Frankfurt", "Zurich", "Prague"]             | 3820   |
| 2    | BF       | Serial | —       | 12.56      | ["Barcelona", "Madrid", "Lyon", "Paris", "London", "Amsterdam", "Frankfurt", "Vienna", "Prague", "Zurich", "Milan", "Rome"]             | 5627   |
| 3    | BF       | Serial | —       | 17.29      | ["Barcelona", "Paris", "Madrid", "London", "Prague", "Frankfurt", "Amsterdam", "Berlin", "Vienna", "Munich", "Zurich", "Geneva", "Lyon", "Milan", "Florence", "Rome"]             | 8685   |
| 4    | BF       | Serial | —       | 18.60      | ["Barcelona", "Paris", "Madrid", "London", "Prague", "Frankfurt", "Zurich", "Lyon", "Amsterdam", "Milan", "Geneva", "Brussels", "Berlin", "Munich", "Vienna", "Budapest", "Zagreb", "Venice", "Florence", "Rome"]             | 11565   |
| 1    | BF       | Parallel | 2       | 0.04445      | ["Madrid", "Barcelona", "Lyon", "Paris", "London", "Frankfurt", "Zurich", "Prague"]             | 3820   |
| 2    | BF       | Parallel | 2       | 12.81      | ["Barcelona", "Madrid", "Lyon", "Paris", "London", "Amsterdam", "Frankfurt", "Vienna", "Prague", "Zurich", "Milan", "Rome"]             | 5627   |
| 3    | BF       | Parallel | 2       | 16.52      | ["Barcelona", "Paris", "Madrid", "London", "Prague", "Frankfurt", "Amsterdam", "Berlin", "Vienna", "Munich", "Zurich", "Geneva", "Lyon", "Milan", "Florence", "Rome"]             | 8685   |
| 4    | BF       | Parallel | 2       | 18.54      | ["Barcelona", "Paris", "Madrid", "London", "Prague", "Frankfurt", "Zurich", "Lyon", "Amsterdam", "Milan", "Geneva", "Brussels", "Berlin", "Munich", "Vienna", "Budapest", "Zagreb", "Venice", "Florence", "Rome"]             | 11565   |
| 1    | BF       | Parallel | 3       | 0.03694      | ["Madrid", "Barcelona", "Lyon", "Paris", "London", "Frankfurt", "Zurich", "Prague"]             | 3820   |
| 2    | BF       | Parallel | 3       | 12.53      | ["Barcelona", "Madrid", "Lyon", "Paris", "London", "Amsterdam", "Frankfurt", "Vienna", "Prague", "Zurich", "Milan", "Rome"]             | 5627   |
| 3    | BF       | Parallel | 3       | 16.98      | ["Barcelona", "Paris", "Madrid", "London", "Prague", "Frankfurt", "Amsterdam", "Berlin", "Vienna", "Munich", "Zurich", "Geneva", "Lyon", "Milan", "Florence", "Rome"]             | 8685   |
| 4    | BF       | Parallel | 3       | 16.32      | ["Barcelona", "Paris", "Madrid", "London", "Prague", "Frankfurt", "Zurich", "Lyon", "Amsterdam", "Milan", "Geneva", "Brussels", "Berlin", "Munich", "Vienna", "Budapest", "Zagreb", "Venice", "Florence", "Rome"]             | 11565   |
| 1    | BF       | Parallel | 8       | 0.04082      | ["Madrid", "Barcelona", "Lyon", "Paris", "London", "Frankfurt", "Zurich", "Prague"]             | 3820   |
| 2    | BF       | Parallel | 8       | 20.47      | ["Barcelona", "Madrid", "Lyon", "Paris", "London", "Amsterdam", "Frankfurt", "Vienna", "Prague", "Zurich", "Milan", "Rome"]             | 5627   |
| 3    | BF       | Parallel | 8       | 15.32      | ["Barcelona", "Paris", "Madrid", "London", "Prague", "Frankfurt", "Amsterdam", "Berlin", "Vienna", "Munich", "Zurich", "Geneva", "Lyon", "Milan", "Florence", "Rome"]             | 8685   |
| 4    | BF       | Parallel | 8       | 16.94      | ["Barcelona", "Paris", "Madrid", "London", "Prague", "Frankfurt", "Zurich", "Lyon", "Amsterdam", "Milan", "Geneva", "Brussels", "Berlin", "Munich", "Vienna", "Budapest", "Zagreb", "Venice", "Florence", "Rome"]             | 11565   |

<p align="right">(<a href="#readme-top">back to top</a>)</p>
