layout: post-math.liquid
title: Solutions to the USC Spring 2018 Code-a-Thon
published_date: 2018-04-25 00:00:00 -0500
description: A very in-depth breakdown of the solutions to the upper division Code-a-Thon problems for the USC Spring 2018 Code-a-Thon. Each solution is thoroughly and rigorously explained with analysis, methodology, and code. Some programming experience is expected, though this was written for the audience of first-year computer science student in mind.
data:
    link_type: internal
    route: feed
    type: blog
    published_date_friendly: April 25th, 2018
---
## Introduction

It isn't necessary to have read [CLRS](https://en.wikipedia.org/wiki/Introduction_to_Algorithms) cover-to-cover to be able to compete in Code-a-Thons, although owning a copy certainly helps. Many of this year's problems were conceived while flipping through these pages, thinking up back stories for questions whose solutions were in pseudocode in front of me. I wrote one fewer question than I had intended to this semester, although only two out of the 71 participants in the upper division were able to complete the problem set, so it would seem the overall competition difficulty was ideal. With one finisher clocking in at just under nine hours (wow!) and the other at just over 23 hours (although analysis of submission times indicates a much deserved sleeping break), the competition provided a difficult challenge that was not insurmountable for our quick witted competitors. I am extremely satisfied with the performance of this semester's participants, and I thank the USC ACM chapter for their efforts in organizing the event, and to the [University of South Carolina](http://sc.edu) and [Krumware](http://krum.io) for their support.

Before we go over the solutions, I will outline the format of the competition. Participants had 24 hours to solve a series of programming challenges. Code-a-Thons are distinctly different from Hack-a-Thons, and are much more akin to programming math competitions. There are several high-profile contests of this nature, including [Google Code Jam](https://code.google.com/codejam/) and [ACM's ICPC](https://icpc.baylor.edu/). The USC Code-a-Thon is split up into 4 divisons, which roughly correspond to 1st semester CS students, 2nd semester CS students, 3rd semester CS students, and 4th (and up) semester CS students. "And up" in the highest division refers to upperclassmen, as well as graduate students. As it would make little sense to expect a first semester freshman to be able to complete the same difficulty challenges as a graduate student, we further divide the divisions into an upper and lower division. Lower division problems often deal with elementary data structures (stacks for example) or with basic algorithms (like binary search). Upper divison problems deal with advanced topics in computer science, including algorithmic design, combinatorics, graph theory, and more. The difficulty gap between these two divisions is quite large. We use [Hackerrank](https://hackerrank.com) to host our competitions, and you can see the problems in their original format by following the links below. If you are interested in these types of problems, I encourage you to visit the problems and attempt them yourself, or to at least ponder their solutions before looking at the ones provided below. Hackerrank allows you to submit code in a variety of languages, and will automatically test your program on a variety of test cases provided by the problem author.

Links to blank contests (best for following along or trying for yourself):

* [Upper Divison Problem Set](https://www.hackerrank.com/usc-acm-spring-2018-upper-divison-problems)

* [Lower Divison Problem Set](https://www.hackerrank.com/usc-acm-spring-2018-lower-divison-problems)

Links to contests (with leaderboards intact):

* [350+, Division 1, Upper Division Problem Set](https://www.hackerrank.com/contests/usc-acm-spring-2018-350-division)

* [240, Division 2, Upper Division Problem Set](https://www.hackerrank.com/contests/usc-acm-spring-2018-240-division)

* [146, Division 3, Lower Division Problem Set](https://www.hackerrank.com/contests/usc-acm-spring-2018-146-division)

* [145, Division 4, Lower Division Problem Set](https://www.hackerrank.com/contests/usc-acm-spring-2018-145-division)

The Github repositories for each division have been made public as well. These contain solutions (sometimes in multiple languages or with multiple approaches), test cases, test case generators, and misc. notes or extra information.

* [Upper Division Problems Repository](https://github.com/z2oh/usc_upperdivision_spring2018)

* [Lower Division Problems Repository](https://github.com/z2oh/usc_lowerdivision_spring2018)

Without further ado...

## Solutions

I will refrain from going over the solutions to the lower division problems, as most of them have fairly straightforward solutions. If there is interest, I may add these solutions at a later date. In the mean time, here are the solutions to the upper division problem set. These are organized in the order in which the problems appeared to contestents, and are approximately in increasing order of difficulty.

#### Problem 1: Leonhard's Libations

Every semester we try to include a trick question where the problem appears to be a programming problem, but the programming solution is incredibly difficult or time-consuming. The best example of one such trick problem required nothing more than knowledge of the [four color theorem](https://en.wikipedia.org/wiki/Four_color_theorem) and 7 characters of Python 2: `print 4`.

This problem is not quite as elegant as that one, but it got the job done.

Several of the mathematically inclined participants immediately recognized the formula in the problem description:

$$\sum_{i=0}^{n} \frac{1}{i!}$$

which converges to [$e$](https://en.wikipedia.org/wiki/E_(mathematical_constant)) when $n = \infty$.

The question asks the programmer to output the value of this formula for various values of $n$ (with $1 \leq n \leq 2^{2^{2^{2^{2}}}}$), **rounded to six decimal places**. The upper bound for $n$ is a 19729 digit decimal number, which gives a hint that performing any real computation with $n$ is infeasible. Let's look at a table of corresponding values of $n$ and $f(n)$ (which is $n$ applied to the formula above):

<center>

| $n$ | $f(n)$ |
| :-- | --: |
| $1$ | $2$ |
| $2$ | $2.5$ |
| $3$ | $2.\overline{6}$ |
| $4$ | $2.708\overline{3}$ |
| $5$ | $2.71\overline{6}$  |
| $6$ | $2.7180\overline{5}$ |
| $7$ | $2.71825396825397$ |
| $8$ | $2.71827876984127$ |
| $9$ | $2.71828152557319$ |
| $10$ | $2.718281801146384$ |
| $11$ | $2.718281826198493$ |
| $12$ | $2.718281828286169$ |

</center>

Okay, and now let's look at the same table with $f(n)$ rounded to six decimal places:

<center>

| $n$ | $f(n)$ |
| :-- | --: |
| $1$ | $2.000000$ |
| $2$ | $2.500000$ |
| $3$ | $2.666667$ |
| $4$ | $2.708333$ |
| $5$ | $2.716667$ |
| $6$ | $2.718056$ |
| $7$ | $2.718254$ |
| $8$ | $2.718279$ |
| $9$ | $2.718282$ |
| $10$ | $2.718282$ |
| $11$ | $2.718282$ |
| $12$ | $2.718282$ |

</center>

Aha! a pattern emerges. As $n$ tends to infinity, $f(n)$ converges to $e$, and with $n > 8$, our approximation of $e$ does not get any better when we limit it to only six decimal places. Since we have only nine possible cases, we can provide a solution to this problem without using any sort math at runtime. In Python 3, this solution might look like this:

```python
line = input()

n = -1
if len(line) == 1:
    n = line

if n == '1':
    print("2.000000")
elif n == '2':
    print("2.500000")
elif n == '3':
    print("2.666667")
elif n == '4':
    print("2.708333")
elif n == '5':
    print("2.716667")
elif n == '6':
    print("2.718056")
elif n == '7':
    print("2.718254")
elif n == '8':
    print("2.718279")
else:
    # Beyond n = 9, n converges to the mathematical constant `e` for 6 decimal
    # places.
    print("2.718282")
```

There were several (subtle) hints to the solution in the problem text. The problem name, "Leonhard's Libations", is a reference to [Leonhard Euler](https://en.wikipedia.org/wiki/Leonhard_Euler), the mathematician for which $e$ is named. Additionally, the sentence, "On any given day, Leonhard knows his limits," (and the many other references to limits) was to indicate that perhaps the limit of the proposed formula should be considered. We had 33 students solve this successfully. Even more recognized that the formula converged to $e$, but they couldn't quite figure out how to express this in code.

#### Problem 2: Pineapple Pack

This problem was the only problem of the hard problem set that I did not write. For an entirely unrelated reason, this was also the problem with which we had the most trouble. I tend to use Python for Code-a-Thons for its concise syntax, expressiveness, and arbitrary precision arithmetic. Not having to worry about integer overflow (or the verbosity of [`BigInteger`](https://docs.oracle.com/javase/7/docs/api/java/math/BigInteger.html)) is a huge boon. I haven't used Python for many floating point programs, since the Code-a-Thon problems I author tend to avoid floating point altogether; using string comparison to check solutions falls short for floating point output when you allow participants to submit code in a [variety of programming languages](http://0.30000000000000004.com/).

My naïveté and crunch-time-coding led me to assume that Python would provide arbitrary precision arithmetic for *non*-integral calculations out of the box. It does not (one must use the [`decimal`](https://docs.python.org/3.0/library/decimal.html) module). More specifically, I assumed that `x // y` was equivalent to `int(x / y)` (in Python, `//` is the integer divison operator. This operator has equivalent functionality to most other language's divison operators; thanks [PEP 238](https://www.python.org/dev/peps/pep-0238/)). Try executing the following line in a Python REPL:

```python
int(9007199254740993 / 1) == 9007199254740993 // 1
```

You will be met with the output `False`. The number in that example is actually the smallest positive integer $x$ where the above statement is `False`. This is because [double precision floating point](https://en.wikipedia.org/wiki/Double-precision_floating-point_format) specifies 53 bits of significand precision. Since $9007199254740993 = 2^{53}+1$, it is not guaranteed to be able to be accurately represented as a floating point number (although $2^{53}+2$ works fine, see the Wikipedia page linked above for the reason why).

Here you came for a solution to a Code-a-Thon and instead you are getting a lecture on IEEE-754. I do apologize for this, but perhaps it will keep you from committing the ultimate Code-a-Thon author's sin: providing incorrect test cases. Fortunately we caught this issue early on, but not before several participants wasted time debugging correct code. This was the only problem that wasn't correct, so I suppose it could have been worse.

Now, how is this problem actually solved?

The problem assumes a completely filled and infinite size [ternary tree](https://en.wikipedia.org/wiki/Ternary_tree), where each node is labeled with an index. The labeling scheme is straightforward, with the root node having index 1, the nodes on the next layer having indices 2, 3, 4, and so on. The first three layers of the tree might look something like this:

<img src="/assets/blog/spring-2018-code-a-thon-solutions/ternary-tree-layers.svg" width="100%" alt="Ternary tree with height three">

This problem asks the programmer to write a program that provides a series of directions to navigate from the root node to some node in tree. The index of this destination node is the input to the program. The series of directions should be a string comprised of the characters `L`, `M`, and `R`, corresponding to left, middle, and right. Looking at the tree above, we can see that input `12` should produce output `RM`. Similarly, input `7` should produce output `LR`.

It seems that a top down approach is unlikely to work, since the size of the tree grows exponentially with each additional layer. Thus, we must start at the specified node and work our way up. To generalize, we need a way to find out which child (left,  middle, or right) that a given node is of its parent. After finding this, we need to find the index of the parent and repeat this process until we get to the root node.

Let's consider a node $i$, where $i$ is the index of the node. How can we find out if $i$ is a left, middle, or right child of its parent? Let's consider the indices of all the left children in the tree above. We have $\\{2, 5, 8, 11\\}$. What about the middle children? $\\{3, 6, 9, 12\\}$. And the right children? $\\{4, 7, 10, 13\\}$. Looking at these three sets we can see a clear pattern. Each right child will have an index one greater than some middle child's index and each left child will have an index one less than some middle child's index. We can also see that each middle child's index is evenly divisible by three. So what would happen if we considered $i \mod 3$? There are three distinct outputs to consider: `0`, `1`, or `2`. If we get `0`, we know that $i$ is a middle child of its parent, since all middle children have an index divisible by three. If we get `1` we know that $i$ is a right child since all right children have an index one greater than a middle child's index (so they are some multiple of three plus one). Lastly, if we get `2` we know that $i$ is a left child since all left children have an index two greater than a middle child's index (or one less than a middle child's index, as $-1 \equiv 2 \pmod{3}$)

So now we know how to determine if a node is a left, middle, or right child of its parent. How do we get the index of the parent? Let's consider each node and its middle child in the above image. We have $\\{(1, 3), (2, 6), (3, 9), (4, 12)\\}$. Each node's middle child has an index exactly three times its own index. To find the index of $i$'s parent, we just have to find the middle child nearest to $i$, take its index, and divide that by three. At this point, we must repeat the process described in the previous paragragh and continue up the tree until we get to the root. At each step up the tree, we record if $i$ was a left, middle, or right child of its parent. When we reach the root node we print out the directions we have recorded in reverse order. We must reverse the order because we recorded the directions from the destination node to the root, but we need to output the directions from the root node to the destination.

There is a single edge case for this problem. The constraints indicate that the input of `1` is a possibility. What should we output in this case? From the context of the problem, it would seem that no directions need to be given to get to this node, since the node with index `1` is the starting node. Sure enough, the expected output for this input is simply the empty string. This was test case 2, and a couple of participants had to put a little bit of extra thought into their solution to account for this edge case.

Let's see some code!

```python
n = int(input())
 
ans = ""
 
# While we are not at the root node...
while n > 1:
    if n % 3 == 0:
        # We have found a middle child
        ans += 'M'
        n = n // 3
    elif n % 3 == 1:
        # We have found a right child.
        ans += 'R'
        # The parent node is the middle child's index divided by three
        n = (n - 1) // 3
    elif n % 3 == 2:
        # We have found a left child.
        ans += 'L'
        # The parent node is the middle child's index divided by three
        n = (n + 1) // 3

# Print the directions that we have recorded in reverse
print(''.join(list(reversed(ans))))
```

We had 30 students correctly solve this problem. Many students skipped straight to this problem since it seemed (and was) more straightforward than the Leonhard's Libations. Many students using C/C++/Java encountered some difficulties because they were using 32-bit integers to read in the input. We carefully chose the input constraint of $2^{63} - 1$ so that the input would fit into an unsigned 64-bit integer. I felt bad watching frustrated students see their (logically correct) code failing on large test cases, but I was once in their position and learning this the hard way kept me from ever making a similar mistake again!

As a Code-a-Thon author, this problem taught me a lot about writing good problems. I should have tested our solution on large (but trivial) test cases. For example, any input of the form $\sum_{i=0}^{h} 3^i$ should simply output a string of $h-1$ `R`s, since we have specified the index of a node all the way to the right of the $h$'th layer. When $h=60$, $\sum_{i=0}^{60} 3^i = 63586737412824305271441649801$. Using this as input to our first (and incorrect) solution, we get output:

```
RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRMLMLLMRMLMRMRRRMMRLLRLRMRLR
```

Gah! If only we had done more testing. Oh well. We will take this as a lesson learned and be more thorough with our testing next time.

#### Problem 3: Pricey Power

This was the first problem with a difficulty rating of hard. The increase in difficulty was reflected in the number of working solutions. Only five participants solved this problem, all of them in division one.

This problem required less raw ingenuity to solve, and more prior knowledge (or [Google-fu](https://en.wiktionary.org/wiki/Google-fu)). For participants well versed in graph theory, all it took was one glance at the provided image for the first example (see below) to realize this question was about [minimum spanning trees](https://en.wikipedia.org/wiki/Minimum_spanning_tree). I will frequently use the abbreviation "MST" to refer to minimum spanning trees.

<img style="display: block; max-width: 20rem; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/pricey_power_test_case_0_path.svg" width="100%" alt="Test case zero path highlighted">

But that's jumping straight to the solution. Let's analyze the problem to figure out exactly what it's asking. SCANA (a real energy company located near Columbia, SC!) has an expensive network of power lines connecting houses in their power grids. Each power line connects exactly two houses to one another, and has an associated monthly cost. This is a perfect problem to model using a [graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)). The edges between nodes have a numerical value associated with them, so we have a *weighted* graph. No power line *generates* money; every power line has a positive cost associated with it. Power can move through lines freely in either direction, so our graph is *undirected*. We are told in the problem description that there can be multiple power grids in each network, meaning two houses may have no connection between them (i.e. no combination of edges in the graph connects those two houses). This means that, at least some of the time, our graph is *disconnected*. Thus, we can model our problem using a *disconnected undirected weighted graph*. For this problem, the input graphs are also simple graphs, meaning that any two nodes have only a single edge between them and that there are no edges connecting a node to itself. In the context of the MST problem, all self-edges can safely be ignored (as they would increase the total cost of a MST without expanding the connected area), and multiple edges between the same node can be eliminated by removing all but the minimum weight edge between two nodes (as the other edges have no chance of making it into a MST because the minimum weight edge between the two nodes will always be the better choice). A versatile MST calculating program should be able to account for non-simple graphs with no extra pre-processing, although this was ultimately unnecessary for this problem, since the input graphs are all simple graphs.

##### Minimum Spanning Trees

So what is a MST anyway? Let's formalize. Let $G = (V, E)$ be a connected graph with a collection of nodes (vertices) $V$ and a collection of edges $E$. Each vertex is identified by a label. Each edge is a 3-tuple containing the labels of the two nodes that the edge connects, and the weight (cost) associated with that edge. Let $G_{ST}$ be a spanning tree (ST) of $G$. $G_{ST}$ must have the following properties:

* $G_{ST} \subseteq E$, *A ST of $G$ is a subset of the edges of $G$. This means that a ST is a collection of edges. A ST is technically a subgraph of $G$ (not a collection of edges), but for our problem we can relax this definition.*
* $|G_{ST}| = |V| - 1$, *There is exactly one fewer edge in a ST of $G$ than there are vertices in $G$. This is because a ST is a [tree](https://en.wikipedia.org/wiki/Tree_(graph_theory)), and trees are acyclic. If there are $|V|$ or more edges, then this means there must be at least one cycle, violating the acyclic property of trees.*

Let $G_{MST}$ be a minimum spanning tree of $G$. Let $ST_{G}$ be the collection of all spanning trees of $G$. $G_{MST}$ must have the following properties:

* $G_{MST} \in ST_{G}$ *A MST of $G$ is also a ST of $G$.*
* $\forall G_{ST} \subseteq ST_{G},$ $\sum_{x \in G_{ST}} x_{w}$ $\geq$ $\sum_{x \in G_{MST}} x_w$ where $x_w$ is the weight of edge $x$. *The weights of the edges in a MST of $G$ must be less than or equal to the sum of weights in any spanning tree of $G$. This means that $G_{MST}$ minimizes the weights of a spanning tree of G, which is where the minimum spanning tree gets its name.*

That was a lot of mathematical notation to describe what is a reasonably simple concept. In short, a minimum spanning tree of a connected graph is a collection of edges such that every node in the graph has a single path to every other node in the graph, and no other collection of edges with this property has a lower sum of weights. Note that I was careful to say **a** MST and not **the** MST. MSTs are not guaranteed to be unique, which is why this problem asks for the sum of weights of a MST rather than some set of edges comprising a MST. All MSTs for a given graph have the same sum of weights. This can be concluded directly from the second condition above.

But wait... this definition is only true for *connected* graphs, and the input for this problem includes *disconnected* graphs. This is important. There are several famous algorithms for calculating MSTs of a graph. The two with which I am most familiar are [Prim's algorithm](https://en.wikipedia.org/wiki/Prim%27s_algorithm) and [Kruskal's algorithm](https://en.wikipedia.org/wiki/Kruskal%27s_algorithm). Out of the box, Prim's algorithm does not work on disconnected graphs. Kruskal's algorithm, however, can be used to calculate a minimum spanning forest without any modifications. The minimum spanning forest is a collection of minimum spanning trees for a disconnected graph: there is one tree in the forest for each connected component of the graph. Prim's algorithm can be modified to calculate a minimum spanning forest (one particpant's submitted solution did this), but I used Kruskal's algorithm to solve this problem.

##### Input Format

Before I describe Kruskal's algorithm, allow me to provide some explanation for the input format of this problem. Consider the following input for a test case:

```
3
0 2 1 5 2 6
1 1 2 4
2 0
```

The first line in this test case means that there are three houses in our power network. The houses are always labeled starting with `0` and going up to `n-1` where `n` is the number of houses in the network. The next `n` lines describe the graph, in what closely resembles an [adjacency list](https://en.wikipedia.org/wiki/Adjacency_list). Each line starts with the label of a house. The next number in the line, `e`, describes the number of *previously unaccounted-for* edges connected to that house (more on that later). The rest of the line consists of `e` pairs of values. The first value of each pair is the label of another house in the network. The second value of each pair is the monthly cost of the power line between the two houses described by the first value in the pair, and the value at the beginning of the line.

To work out the example test case above, we can see there are two *previously unaccounted-for* edges that connect to house `0`. One of these edges connects to house `1` and has a weight of `5`. The other connects to house `2` and has a weight of `6`. Onto the next line...

House `1` has one *previously unaccounted-for* edge. This edge connects to house `2` and has a weight of `4`. We know that house `1` also has an edge connecting it to house `0`. This is where the *previously unaccounted-for* part comes into play. In a standard adjacency list for an undirected graph, every edge is present in the list twice: once for each node that it connects. As its input, Kruskal's algorithm takes a list of every edge. If the test cases for this problem were standard adjacency lists, the programmer would have to implement some extra pre-processing steps to remove the duplicates. This isn't necessarily very difficult, but it is more convienent to read in each edge exactly once.

Finally, house `2` has `0` unaccounted-for edges, since the edges to house `0` and house `1` have already been accounted for on previous lines. In every test case, the last house of the test case will always have `0` unaccounted-for edges.

Each input may have more than one test case (a test case looks like the example above). The first line of each input specifies the number of test cases. This is done to prevent participants from submitting simple programs like `print(10)` to try to get some free points on any test cases where the output might just be `10`.

##### Disjoint-set Forests

Before we discuss Kruskal's algorithm, we must first discuss the data structure that lies at its core: the [disjoint-set forest](https://en.wikipedia.org/wiki/Disjoint-set_data_structure). The disjoint-set forest is a data structure that can be used to keep track of sets of items. Kruskal's algorithm treats connected componenets of graphs as sets, with the nodes in these components being the members of the set. If there exists some path (series of edges) between two nodes in the graph, then these two nodes are part of the same set. The disjoint-set forest has three essential operations: $MakeSet$, $Find$, and $Union$. 

$MakeSet$ makes a new *set* which is kept track of internally as a tree. Each node of this tree is a member of the same set. Each node of the tree has a couple of properties: a pointer to its parent node, and a numerical value called the rank of the node (more on that later). Calling $MakeSet$ will initialize a set with a single node in its tree. This node should be initialized with rank `0` and with the parent property pointing to itself.

$Find$ returns a *representation* of a given set. This means that if we have two items of the same set $x, y$, then $Find(x) = Find(y)$. But what does *representation* of a set mean? A representation of a set is just a specific member of the set. It is important that each member of the set return the same representative element when we call $Find$ on it. Since we keep track of our set as a tree, our set representative is the root node of the tree. Thus calling $Find$ on a node in our set just traverses the tree up to the root node and returns the root. This ensures that each node in the set has the same representative, since a tree has only one root node.

$Union$ combines two sets together. Let's say we have two sets that we want to union: $A$ and $B$ ($A$ and $B$ are just members of sets, but we can call $Find(A)$ and $Find(B)$ to get the root nodes of those sets). If we call $Union$ on these sets, we want them both to be part of the same tree. There are two ways we can do this: make the root node of $A$'s tree have a parent pointer pointing to some node in $B$'s tree, or make the root node of $B$'s tree have a parent pointer poining to some node in $A$'s tree. For correctness sake, it doesn't actually matter which one you pick. No matter which you choose, the root node of the resulting tree will still be the same for every element of the set (thus for any elements $x,y \in A \cup B$, we have $Find(x) = Find(y)$). For now, let's say that $Union(A,B)$ will always make $B$'s tree's root node parent pointer point to the root element of $A$'s tree. I will give an example shortly to show why this is not ideal.

One of the most common uses of disjoint-set forests is to find connected components in a graph. Let's do that, considering the following unweighted undirected graph:

<img style="display: block; max-width: 30rem; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/disjoint_set_example.svg" width="100%" alt="Example graph for disjoint-set forest">

Just by looking at the image, we can see that this graph has two connected components: $\\{A, B, C, D, E\\}$ and $\\{F, G, H\\}$. Of course, for larger graphs with many intersecting edges, this task becomes impossible without the aid of computers. We start by calling $MakeSet$ on each node in our graph. Each node in the graph becomes the root node in a tree representing the set containing that node. Each tree node has the same label as the corresponding node in the above graph, and each node's parent pointer is initialized to point to the node itself. Our current disjoint-set forest looks like this:

<img style="display: block; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/disjoint_set_example_one.svg" width="100%" alt="Example graph for disjoint-set forest">

Now we loop over every edge in our graph above. The order in which we loop doesn't matter if we are just looking for connected components. Let's start with the edge between $A$ and $B$. We know that $A$ and $B$ are in the same set, because there exists an edge between them. We call $Find(A)$ and $Find(B)$ and compare these values to discover that they are different ($Find(A)$ returns $A$ and $Find(B)$ returns $B$). Because we know they are connected (there exists an edge between them) but our $Find$ calls say otherwise, we call $Union(A,B)$ to merge the two sets. After our call to $Union$, our disjoint-set forest looks like this:

<img style="display: block; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/disjoint_set_example_two.svg" width="87.5%" alt="Example graph for disjoint-set forest">

Great! Note that $Find(A)$ still returns $A$, but $Find(B)$ now also returns $A$, which indicates to us that $A$ and $B$ are in the same set. Now we will consider the edge between $C$ and $D$. Just like before, we check if $Find(C) = Find(D)$. It doesn't, but we know these nodes are in the same set, so we call $Union(C, D)$. Our disjoint-set forest now looks like this:

<img style="display: block; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/disjoint_set_example_three.svg" width="75%" alt="Example graph for disjoint-set forest">

Now things start to get interesting... let's consider the edge between $B$ and $C$. $Find(B)$ returns $A$ and $Find(C)$ returns $C$, so we know we need to call $Union$. When we call $Union(B, C)$ we set $C$'s parent pointer to point to $B$. We get the following:

<img style="display: block; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/disjoint_set_example_four.svg" width="62.5%" alt="Example graph for disjoint-set forest">

Now we are getting somewhere; $Find(A)$ $=$ $Find(B)$ $=$ $Find(C)$ $=$ $Find(D)$ $=$ $A$; we can easily verify that any of these nodes are part of the same connected component in our graph by calling $Find$ on them and comparing the values. With the way we built the tree, each call to $Find$ runs in $\mathcal{O}(n)$ time where $n$ is the number of nodes in each connected component. That's not great... fortunately there are some optimizations that let us get this down to nearly constant time. I say *nearly* because the amount of time taken does actually grow with respect to $n$. Specifically, the amortized cost of these operations are in $\mathcal{O}(\alpha(n))$, where $\alpha$ is the [inverse Ackermann function](https://en.wikipedia.org/wiki/Ackermann_function#Inverse). However, if we can guarantee that our input graph has fewer nodes in it than the number of atoms in the universe, then $\alpha(n)$ will always be less than 5. So for any practical input, the time complexity of these operations are in $\mathcal{O}(5) = \mathcal{O}(1)$, so we can guarantee constant time with this input size restriction. I will discuss these optimizations shortly. Fow now, let's finish our naïve disjoint-set forest.

Let's look at the edge between nodes $A$ and $D$. $Find(A)$ returns $A$ and $Find(D)$ returns $A$ as well. This means that nodes $A$ and $D$ are already in the same set, so we don't have to do anything. Onto the next edge! Let's consider the edge between nodes $A$ and $E$. These $Find$ calls returns different values, so we call $Union(A, E)$. Here is the resulting disjoint-set forest:

<img style="display: block; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/disjoint_set_example_five.svg" width="50%" alt="Example graph for disjoint-set forest">

Looking good. When we process the edge between $D$ and $E$ we find that they are already in the same set ($Find(D)$ $=$ $Find(E)$), so we do nothing. For brevity's sake, I will perform the remaining construction with no explanation. The methodology used is exactly the same as it was in the preceeding paragraphs. Our final naïve disjoint-set forest looks like this:

<img style="display: block; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/disjoint_set_example_six.svg" width="37.5%" alt="Example graph for disjoint-set forest">

This isn't ideal though. $Find(D)$ still takes 4 steps through the tree to return $A$ (linear time). We want our disjoint-set forest to look like this:

<img style="display: block; margin: auto;" src="/assets/blog/spring-2018-code-a-thon-solutions/disjoint_set_example_seven.svg" width="50%" alt="Example graph for disjoint-set forest">

This way each call to $Find$ will take no more than 2 steps through the tree in the worst case (constant time). So how do we achieve a disjoint-set forest like this? We use the optimizations that I mentioned earlier. These optimizations are known as **path compression** and **union by rank**. Here is where the nodes' rank variable that I mentioned earlier comes into play. The path compression optimization is implemented in the $Find$ function, and the union by rank optimzation is implemented (unsurprisingly) in the $Union$ function. We will discuss path compression first.

###### Path Compression

The naïve way of implementing $Find$ might look something like this:

```
Find(node)
    if node.parent == node
        return node
    else
        return Find(node.parent)
```

If a node's `node.parent` property is pointing to itself, then we have reached the root of the tree and can return the node. Otherwise, we return the result of calling $Find$ on the parent in order to step up the tree one level repeatedly until we get to the root node. But what if we can mutate our tree while we traverse up through it? Each node's parent needs only to point to the root of the tree. We can change the parent pointers of each node along the path to the root to point directly to the root node. We are *compressing* the paths from the query node (and all nodes on the way up) to point directly to the root node. This improved implementation might look like this:

```
Find(node)
    if node.parent == node
        return node
    else
        node.parent = Find(node.parent)
        return node.parent
```

On subsequent calls to $Find$ with the same query node, we now return in constant time. As an added bonus, calling $Find$ on any nodes along the path from the query node to the root also return in constant time. Awesome! Now let's consider union by rank.

###### Union by Rank

The naïve way of implementing $Union$ might look something like this:

```
Union(A,B)
    Find(B).parent = A
```

We are making the root node of the set in which $B$ is a member point to $A$ as its parent, essentially splicing $B$'s set into $A$'s set. We can make a very easy optimization that will reduce the height of our trees dramatically:

```
Union(A,B)
    Find(B).parent = Find(A)
```

This will cause $B$'s tree to get added just below the root of $A$'s tree, shortening the paths from the nodes of $B$ to $Find(A)$. This is a big improvement, but we can still do better. In general, we want our trees to be as short as possible, to reduce the number of steps that each $Find$ operation takes to get to the root. Let's say the tree with node $B$ has height 5 and the tree with node $A$ has height 3. If we set $B$'s tree's parent pointer to point to the root of $A$'s tree, then our resulting tree has height 6 (the height of $B$'s tree plus one more for the new tree root). However, if we set $A$'s tree's parent pointer to point to the root of $B$'s tree, then our resulting tree has height 5 (since the height of $A$'s tree was less than the height of $B$'s tree, so $B$'s tree still has the same height). We can achieve shorter trees by being smarter about which tree we append to the other.

So how can we be do this? We want to always append the shorter tree to the taller tree to avoid increasing the overall tree height. By keeping track of the maximum heights of each tree, we can accomplish this. The rank variable on each node is used to keep track of this maximum possible height. We call this variable *rank* instead of *height* because the actual height of a tree can be changed via path compression; *height* would be misleading, as this variable does not represent the actual height of the tree, only the maximum possible height.

Recall that the rank of each node is initialized to `0`. We change our $Union$ operation to the following:

```
Union(A,B)
    rootA = Find(A)
    rootB = Find(B)
    if rootA.rank < rootB.rank
        rootA.parent = rootB
    else if rootA.rank > rootB.rank
        rootB.parent = rootA
    else if rootA.rank == rootB.rank
        rootB.parent = rootA
        rootA.rank = rootA.rank + 1
```

There are three conditions here. If the rank of the set containing $A$ is less than the rank of the set containing $B$, then we change $A$'s tree's root parent pointer to point to $B$'s tree's root, as the tree containing $A$ is shorter than the tree containing $B$. If the rank of the set containing $B$ is less than the rank of the set containing $A$, we do the opposite: appending the tree containing $B$ to the root of the tree containing $A$. If the two ranks are equal, there is nothing clever we can do. We simply append one tree to the other and then increment the rank of the new root node (as the full tree's height now increased by one). Union by rank prevents us from growing our trees in a linear fashion like we did in the example above.

Together, path compression and union by rank improve the amortized time complexity for our $Find$ and $Union$ operations from $\mathcal{O}(n)$ to $\mathcal{O}(\alpha(n)$, which is just $\mathcal{O}(1)$ for any practical $n$. I will not present the analysis that leads to these results, but the third edition of CLRS covers it in depth in section 21.4 (it takes them eight and a half pages).

##### Kruskal's Algorithm

Now that we understand disjoint-set forests, let's discuss Kruskal's algorithm. Don't worry; we are almost there. The disjoint-set forest does all the heavy lifting for us. Let's say we have the nodes of our graph in a list called `nodes` and the edges of our graph in a list called `edges`. Each edge is a 3-tuple and has the following three datums: the label of a node $A$, the label of a node $B$, and a weight. The edge connects node $A$ to node $B$ with the specified weight. The entire pseudocode for Kruskal's algorithm is as follows:

```
mstEdges = {}
for each node in nodes
    MakeSet(node)
sort edges by weight
for each edge in edges
    setA = Find(edge.A)
    setB = Find(edge.B)
    if setA != setB:
        Union(setA, setB)
        mstEdges.add(edge)
```

That's it! At the end of this code block, `mstEdges` contains exactly the edges of a minimum spanning tree on our graph. Let's break this code down. We start out by declaring an empty set of edges; we will add the edges of our MST to this set as we come across them. Next up, we call $MakeSet$ on each node in our graph. This will initialize a new set tree for each node in our graph. Then we sort the edges by weight in nondecreasing order. The edge with the lowest weight is in the first index of `edges`, and the edge with the greatest weight is in the last index of `edges`. We then process each edge in our graph. We call $Find$ on the two nodes that the edge connects. If these two calls to $Find$ do not return the same set representative, then we have found an edge in our MST. This is the lowest weight edge that connects `setA` to `setB`. We $Union$ the two sets together, and add our edge to our `mstEdges` set.

For this problem, we don't actually need to keep track of the entire MST, just the sum of the weights. So the pseudocode for the solution to this problem might look like this:

```
sum = 0
for each node in nodes
    MakeSet(node)
sort edges by weight
for each edge in edges
    setA = Find(edge.A)
    setB = Find(edge.B)
    if setA != setB:
        Union(setA, setB)
        sum = sum + edge.weight
```

I really like Kruskal's algorithm because of its simplicity. All we really do is sort the edges in nondecreasing order and keep track of the connected components as we add each edge to our MST. Most of the complexity exists in the implementation of the disjoint-set forest, which is very good at finding spanning trees out of the box. For a graph with $n$ edges, the running time of Kruskal's algorithm is $\mathcal{O}(n \lg n)$, which is just the time it takes to sort the edges (this isn't quite true. If we have 0 edges and $m$ nodes in our graph, we still call $MakeSet$ $m$ times. Our real runtime is $\mathcal{O}(m + n \lg n)$, but for any *interesting* graph the $n \lg n$ portion dominates).

If you want to learn more about Kruskal's algorithm, I recommend reading section 23.2 of the third edition of CLRS. Chapter 21 of the third edition of CLRS also covers disjoint-set data structures in more detail than presented here.

##### Code Solution

The full Python code for my solution is below:

```python
# With path compression and union by rank, performing N operations on the
# DisjointSet will result in a running time that is almost linear on N. While
# it is strictly superlinear, in the case of this problem the input constraints
# can be used to place a linear upper bound on the running time.
class DisjointSet:
    # We make sure that the node's parent is set to itself to start, and that
    # the rank of the node is 0.
    def make_set(x):
        x.p = x
        x.rank = 0

    # We link the two nodes that are representative of the set for x and y.
    def union(x, y):
        DisjointSet.link(DisjointSet.find_set(x), DisjointSet.find_set(y))

    # We link the two nodes x and y.
    def link(x, y):
        # Here we are using union by rank to keep the asymptotic complexity
        # as low as possible.
        # If x's rank is greater than y's, y is set as a child of x.
        if x.rank > y.rank:
            y.p = x
        else:
            x.p = y
            # If the ranks are equal, we must increase the rank of the new
            # parent (y).
            if x.rank == y.rank:
                y.rank = y.rank + 1

    # We find the representative node for x, which is the root node for the
    # tree of which x is a member.
    def find_set(x):
        if x != x.p:
            # Here we perform path compression by making sure that the root
            # node is the parent of as many nodes in the tree as possible,
            # which speeds up future accesses.
            x.p = DisjointSet.find_set(x.p)
        return x.p

# The Node object. This is a node in our DisjointSet, not a node in our input
# graph. It is initialized with the id of the vertex in our input graph.
class Node:
    def __init__(self, id):
        self.id = id

# T is the number of test cases.
T = int(input())

# Iterate over our test cases
for _ in range(T):
    # num is the number of vertices in our graph.
    num = int(input())

    # We maintain a set of the vertices, to make sure we have the ids for 
    # each one.
    vertices = set()

    # We also maintain a set of the edges in our graph.
    edges = set()

    # Our input gives us `num - 1` lines, with each line containing the leading
    # edge id, followed by the number of edges, and then a list of trailing
    # edge ids and the edge weights.
    # For example 5 2 7 11 9 10 means:
    # There are two edges connecting with vertex 5, an edge to vertex 7 (with
    # weight 11) and an edge to vertex 9 (with weight 10).
    for _ in range(num - 1):
        line = input().split(' ')
        leading = line[0]
        numEdges = int(line[1])
        # For each edge specified in our input line
        for i in range(numEdges):
            # We add 2 to all indices to account for the leading edge and
            # number of edges in our input line.
            trailing = line[2 + i * 2]
            # Add the leading and trailing edges to the set to make sure we get
            # every vertex.
            vertices.add(leading)
            vertices.add(trailing)
            # We add the edge to our edges set. An edge is represented by a
            # 3-tuple, (leading edge id, trailing edge id, weight)
            # Since each edge in specified in our line has two space separated
            # pieces, we can index to the edge specified with i * 2. We again
            # add 2 to index past the leading edge id and number of edges, then
            # add 1 to index to the weight.
            edges.add((leading, trailing , int(line[2 + i * 2 + 1])))

    # We will store our disjoint set forest in a dictionary. The keys of the
    # dictionary will be the vertex ids, and the values will be the nodes of
    # our disjoint-set trees.
    forests = {}
    for vertex in vertices:
        # Create a node with the vertex and assign it to the key `vertex`,
        # which is the id of the vertex.
        forests[vertex] = Node(vertex)
        # We call `make_set` on the node to assign its parent as itself and
        # set its rank to 0.
        DisjointSet.make_set(forests[vertex])

    # Now we convert our edges set into a list, and sort it by edge weight.
    edges = list(edges)
    edges.sort(key = lambda edge: edge[2])

    # We iterate over every edge. `total` keeps track of the sum total of all
    # the weights of the edges that we add to our MSP.
    total = 0
    while len(edges) > 0:
        edge = edges.pop(0)
        # We call find_set on the leading and trailing ids of the edge in
        # question.
        set_leading = DisjointSet.find_set(forests[edge[0]])
        set_trailing = DisjointSet.find_set(forests[edge[1]])

        # We have found an edge that connects two componenets of our graph.
        # This edge should be added to the MSP weight total.
        if set_leading != set_trailing:
            # We must now union the nodes for our leading and trailing ids for
            # this edge. Path compression and union by rank are used in this
            # implementation of disjoint set forests to ensure maximum
            # efficiency.
            DisjointSet.union(forests[edge[0]], forests[edge[1]])
            total += edge[2]

    # Print out our total. On to the next test case!
    print(total)
```

To generate the input graphs, I wrote a small [Rust program](https://github.com/z2oh/rgg). I hope to expand this program later to have more functionality. I'm sure I will write another Code-a-Thon problem that uses graphs at some point, so this will come in handy.

#### Problem 4: Tingle Towers
It is a USC Code-a-Thon tradition to include [Tingle](https://en.wikipedia.org/wiki/Tingle) as backstory for at least one problem, thus the story for this problem was born. Slightly more students solved this problem successfully than the previous one (six versus five), but I suspect this was because the question was easier to Google. The only valid solution for division 2 included this comment:

```python
# Borrowed kindly from https://www.geeksforgeeks.org/lcs-longest-common-subsequence-three-strings/
```

Oh well! We encourage Googling during the competition for two main reasons: we have no way to enforce a no-Googling policy (as students can participate from home or elsewhere), and the ultimate goal of our Code-a-Thons is to expose participants to more problems or types of problems in computer science, as well as getting them to practice implementing them in a competitive setting. As a participant in the USC Code-a-Thon several years ago, I learned about the [matrix exponentiation](https://www.nayuki.io/page/fast-fibonacci-algorithms) method for rapidly calculating [Fibonacci numbers](https://en.wikipedia.org/wiki/Fibonacci_number). I implemented this by following pseudocode I found online, and I have not forgotten how it works to this day. My only hope is that the student whose code included the above comment took the time to read the article and understand how the code worked instead of blindly copying and pasting. For the Code-a-Thon in which USC annually participants ([ACM's ICPC](https://icpc.baylor.edu/)), there is no internet access. We want students to Google solutions now so that they will remember them later.

##### Longest Common subsequences

Anyway, on to the problem. If you read the above code comment you have probably figured out that this problem boils down to finding the longest common subsequence of three strings. Each test case presents three strings made up of the characters `R`, `G`, and `B`. The goal is to find the length of the longest sequence of `R`s, `G`s, and `B`s such that this sequence is a subsequence of every input string. Before diving into the solution, let's formalize our understanding of a [subsequence](https://en.wikipedia.org/wiki/Subsequence).

First of all, a [sequence](https://en.wikipedia.org/wiki/Sequence) is some collection of elements where the order of the elements in the collection is important. A subsequence is a sequence built from some other subsequence by removing zero or more elements from the original sequence (but preserving the relative order between all other elements). For example, the sequence $\langle 1, 2, 3 \rangle$ has many subsequences: $\langle 1, 2, 3 \rangle$ (obtained by removing no elements from the original sequence), $\langle 1, 2 \rangle$, $\langle 2, 3 \rangle$, $\langle 1, 3 \rangle$, $\langle 1 \rangle$, $\langle 2 \rangle$, $\langle  3 \rangle$, and $\langle \rangle$ (the empty sequence, obtained by removing all of the elements from the original subsequence).

A [common subsequence](https://en.wikipedia.org/wiki/Subsequence#Common_subsequence) of two sequences $A$ and $B$ is some sequence that is both a subsequence of $A$ and a subsequence of $B$. A longest common subsequence $LCS$ of two sequences $A$ and $B$ is some common subsequence of $A$ and $B$ such that there exists no other common subsequence of $A$ and $B$ that has more elements than $LCS$. These can be generalized from two subsequences to arbitrarily many subsequences.

For example, given the sequences $A = \langle 1, 2, 3, 1, 2, 3 \rangle$, $B = \langle 1, 1, 1, 1, 2, 2, 1, 1 \rangle$, $C = \langle 1, 1, 2, 2, 2, 3 \rangle$, a $LCS$ of $A$, $B$, and $C$ is $\langle 1, 1, 2 \rangle$. Note that this is not the only $LCS$ of $A$, $B$, and $C$. For example, $\langle 1, 2, 2 \rangle$ also fits the critera. Both of these have length three, and there is no length four subsequence that is common to $A$, $B$, and $C$.

##### naïve solution

The naïve algorithm to solve this problem finds every possible subsequence of the shortest input string, and checks each one for its existence in the other two input strings. If the subsequences are calcuated in decreasing order of length (starting with the entire sequence, then all subsequences with length one shorter than the sequence, etc.), then as soon as we find a match we return right away. If we exhaust all possible subsequences, then we return the empty sequence. This algorithm is a fairly intuitive brute force solution. The time it takes for this solution to solve the problem grows with the number of subsequences of the shortest input string. For each subsequence, we do a linear amount of work to check if the subsequence is a subsequence of the other input strings. So how many subsequences does a sequence of length $n$ have?

We note that the sequence of length $0$ has exactly one subsequence: the empty sequence. Let us consider a sequence $S$ of length $k$. $S$ has some unknown number of subsequences $n$. Now consider some element $x$. We append the element $x$ to the sequence $S$ to make a new sequence $S'$ of length $k+1$. We note that all $n$ subsequences of $S$ are still subsequences of $S'$, because a subsequence is made by removing some arbitrary number of elements from the sequence, so the element $x$ can be removed in each of these cases. We also note that the *new* subsequences of $S'$ (those that were not subsequences of $S$) are *exactly* the subsequences of $S$ with $x$ appended to the end. In other words, for any subsequence $Q$ of $S'$, $Q$ is either exactly a subsequence of $S$ or some subsequence of $S'$ with the element $x$ at the end, which is exactly some subsequence of $S$ with $x$ appended to it. This means that by adding $x$ to our sequence $S$, we have doubled the number of subsequences in $S'$. In general, a sequence of length $k+1$ has exactly two times the number of subsequences as a sequence of length $k$. Since the sequence of length $0$ has one subsequence, this means that a sequence of length $n$ has $2^{n}$ subsequences.

Thanks to [mathematical induction](https://en.wikipedia.org/wiki/Mathematical_induction), we know that our naïve solution is in $\mathcal{O} (m \cdot 2^{n})$ where $n$ is the length of the shortest input sequence and $m$ is the length of the longest input sequence. For $n = m = 100$ (the upper bound for this problem) this algorithm would take just over 4000 years to produce an answer if we used all of the computing power in the world (using [this 2015 estimate](https://aiimpacts.org/global-computing-capacity/)). This calculation makes the incorrect assumption that each operation of our algorithm is a floating point operation, but our time estimate is accurate enough for us to realize that clearly this is not the correct approach.

Let's break down the problem and figure out a better way to solve it.

##### Analysis of LCS Problem

For ease of analysis, we will assume the LCS problem with only two input strings (rather than the three provided by this problem). We will generalize later to $n$-dimensions, and then use $n=3$ to write a solution. Because this problem asks for the length of a longest common subsequnce, we just need to find *any* LCS of our inputs to figure out our answer (the length of the LCS).

Let's try working through our sequences backwards. Consider two sequences $A$ and $B$, of lengths $n$ and $m$. I will use the notation $S_{x..y}$ to indicate a contiguious subsequence of $S$ from index $x$ to index $y$. For example: $A_{1..n}$ is the entire sequence $A$, and $A_{2..n-1}$ is the subsequence of $A$ with the first and last elements removed. If the last element of $A$ is the same as the last element of $B$ then we know this element will always be in a LCS of $A$ and $B$. We can safely remove the last elements from $A$ and $B$ and consider these truncated sequences with their last elements removed. If we find a LCS of these two truncated sequences, then we can simply append the element that we had previouly removed to the end of that LCS to get a LCS of $A$ and $B$. So when the last two elements of $A$ and $B$ are the same, $LCS(A_{1..n}, B_{1..m})$ $=$ $LCS(A_{1..n-1}, B_{1..m-1})$ $+$ $A_{n}$. This seems useful, but it only works when the last elements of our sequences are the same. What do we do if the last element of $A$ and the last element of $B$ are different? Since the last elements of $A$ and $B$ are different we know that they cannot both be in the same LCS (because if one is the last element of a LCS, the other cannot be the last element of the same LCS as it is different). Thus we have three cases:

* $A_n$ is in a LCS of $A$ and $B_{1..m-1}$ $\implies$ $LCS(A, B)$ $=$ $LCS(A, B_{1..m-1})$
* $B_m$ is in a LCS of $A_{1..n-1}$ and $B$ $\implies$ $LCS(A, B)$ $=$ $LCS(A_{1..n-1}, B)$
* $A_n$ and $B_m$ are not in any LCS of $A$ and $B$ $\implies$ $LCS(A, B)$ $=$ $LCS(A_{1..n-1}, B_{1..m-1})$

Now we are getting somewhere. In general, any time that we are recursively reducing the size of our input we are making progress towards a solution. If we can combine these three rules with the above rule (when $A_n$ is the same as $A_m$) then we can eventually reduce our problem to $LCS(A', B')$ where either $A'$ or $B'$ is the empty sequence. $LCS(A', B')$ is just the empty sequence, so when we reach this point we can start unwinding our recursion and use the $ + A_{n}$s above to figure out a LCS. Sweet.

We know that one of the above three cases must be true when $A_n$ and $B_m$ are different, since if all cases are false it would imply that $A_n$ and $B_m$ are both in some LCS, which we have already proven cannot happen. But how do we know which of these statements is true, and thus which path of recursion to travel down? In short: we can't. There is no way to know which of these is true without figuring out each one. This seems like a problem... when the last elements of our truncated sequences do not match we are making three recursive calls but only reducing our input size to each one by a single element. We can model this behavior as a recurrence relation:

\begin{aligned}
T(n) &= 3 \cdot T(n - 1) + 1 \\\\
T(n) &= 3 \cdot (3 \cdot T(n - 2) + 1) + 1 \\\\
T(n) &= 9 \cdot T(n - 2) + 2 \\\\
T(n) &= 9 \cdot (3 \cdot T(n - 3) + 1) + 2 \\\\
T(n) &= 27 \cdot T(n - 3) + 3 \\\\
\dots \\\\
T(n) &= 3^{n} + T(n - n) + n \\\\
T(n) &= 3^{n} + 1 + n \\\\
\Downarrow \\\\
T(n) &\in \mathcal{O} (3^n) \\\\
\end{aligned}

Oh no! Our recursive solution doesn't appear to be any better than the naïve one! Fear not: we are closer than it appears.

First of all, we have shown that the solution to an instance of the LCS problem can be obtained from the combination of the solutions to that instance's sub-problems. This is known as **optimal substructure**, and is an important property to look for when attempting to solve a problem. During our recursion, we are performing a lot of duplicate work. Consider the following example: we are calculating $LCS(A, B)$ where $A$ has length $10$ and $B$ has length $10$. Let's look at the sub-problem $LCS(A_{1..5}, B_{1..5})$. This sub-problem is called in the $LCS(A_{1..6}, B_{1..5})$ subproblem, but also in the $LCS(A_{1..5}, B_{1..6})$ and $LCS(A_{1..6}, B_{1..6})$ sub-problems.This is known as **overlapping sub-problems**. Whenever we have both **optimal substructure** and **overlapping sub-problems**, we can apply [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming) to speed up our algorithm.

##### Dynamic Programming Solution

The key concept of dynamic programming is recording the results of the overlapping sub-problems so that the next time they are called, we can look up our already-computed answer instead of recalculating it. This concept is known as [memoization](https://en.wikipedia.org/wiki/Memoization), and is a powerful tool in preventing algorithms from doing duplicate work. Often we implement memoization as a table; each entry of the table corresponds to a specific sub-problem of the inital problem. For the LCS problem, the sub-problems are each possible combination of truncated sequences of $A$ and $B$. If $A$ has length $n$, there are $n$ possible truncated sequences of $A$. If $B$ has length $m$, there are $m$ possible truncated sequences of $B$. This means that altogether we have $n \times m$ total sub-problems. It makes sense to use a two-dimensional table for the LCS problem applied to two inputs. As we move $/rightarrow$ in our table, we increase the number of elements in sequence $A$. As we move $/downarrow$ in our table, we increase the number of elements in sequence $B$. Tables are easier to visualize on computers than they are in our heads. Fortunately for both of us, you happen to be looking at one.

<style>
    .lcs-table table {
        border-spacing: 0px;
    }
    .lcs-table td, .lcs-table th {
        text-align: center;
        width: 5rem;
        height: 5rem;
        padding: 1rem;
        border-right: 1px solid #e1e1e1;
        border-bottom: 1px solid #e1e1e1;
    }
    .lcs-table td:last-child, .lcs-table th:last-child {
        border-right: 0;
    }

    .lcs-table tr:last-child td {
        border-bottom: 0;
    }
</style>

<center class="lcs-table" style="overflow: auto;">

|   | $1$  | $2$  | $\cdots$  | $n$  |
|---|---|---|---|---|
| $1$  | $LCS(A_{1..1}, B_{1..1})$ | $LCS(A_{1..2}, B_{1..1})$ | $\cdots$ | $LCS(A_{1..n}, B_{1..1})$  |
| $2$  | $LCS(A_{1..1}, B_{1..2})$ | $LCS(A_{1..2}, B_{1..2})$ | $\cdots$ | $LCS(A_{1..n}, B_{1..2})$  |
| $\vdots$ | $\vdots$ | $\vdots$ | $\ddots$ | $\vdots$ |
| $m$  | $LCS(A_{1..1}, B_{1..m})$ | $LCS(A_{1..2}, B_{1..m})$ | $\cdots$ | $LCS(A_{1..n}, B_{1..m})$ |

</center>

We have used $LCS(A, B)$ a lot, but haven't really gone over what this function actually *returns*. This depends on what you want to know. If you are looking for the length of the LCS of $A$ and $B$, then $LCS(A, B)$ just returns the length of the LCS. If you are looking for an actual subsequence, LCS has to return something other that just the length. Despite not needing to find an actual subsequence for this problem, I will provide the algorithm for finding it, since it doesn't take much additional work.

"*Wait a second...*" you may be thinking. "*This table only accounts for two inputs! The problem we are solving has three!*" Yes, yes, I know. I haven't forgotten about the $n$-dimensional generalization. Unfortunately, HTML cannot easily render $n$-dimensional tables. The intuition for 2-dimensional vs. 3-dimensional vs. $n$-dimensional is very similar, so I will explain the algorithm for two dimensions (with visualizations) and then I will explain the formulation for the $n$-dimensional version. Finally, I will provide code for the 3-dimensional version. Don't fret! Back to the algorithm:

There are two main approaches for solving dynamic programming problems. The *top-down* approach attempts to delay filling out entries in the table until they are needed. The *bottom-up* approach fills out the entire table first before attempting to solve the actual problem. In my experience, *bottom-up* is usually the easier, or more intuitive approach. The *top-down* approach sometimes involves many constant-time duplicate table lookups which can impart a performance penalty. With *bottom-up* we have more control over the order in which we fill out the table. That said, both approaches are equivalent in terms of their validity. I will present the bottom-up approach for this problem: we will start filling out our table in the top left (when $n = m = 1$) and work our way to the bottom left (when $n$ and $m$ are at their maximal values). When we finish filling out the table, our answer will be sitting in the bottom left entry of our table, ready to be plucked out and returned.

What do we need in an actual table entry? There are two important pieces of information we need to store: the length of the LCS at that point and a direction that points toward the previous element in the LCS. The direction aspect will make more sense shortly. Let's work out an example, filling out a table as we go.

Our input sequences will be [`KERNIGHAN`](https://en.wikipedia.org/wiki/Brian_Kernighan) and [`RITCHIE`](https://en.wikipedia.org/wiki/Dennis_Ritchie). Let's make a blank table. In this example, we are going to zero-index our sequences and table to make it easier to convert to code later. Here is our empty table:

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` |     |     |     |     |     |     |     |     |
| `K` |     |     |     |     |     |     |     |     |
| `E` |     |     |     |     |     |     |     |     |
| `R` |     |     |     |     |     |     |     |     |
| `N` |     |     |     |     |     |     |     |     |
| `I` |     |     |     |     |     |     |     |     |
| `G` |     |     |     |     |     |     |     |     |
| `H` |     |     |     |     |     |     |     |     |
| `A` |     |     |     |     |     |     |     |     |
| `N` |     |     |     |     |     |     |     |     |

</center>

We have all possible combinations of subsequences of our two inputs here. Note that this includes the empty sequence. We know that the length of the longest LCS between the emtpy sequence and any sequence is 0, so we fill in these entries with a 0. 

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ |     |     |     |     |     |     |     |
| `E` | $0$ |     |     |     |     |     |     |     |
| `R` | $0$ |     |     |     |     |     |     |     |
| `N` | $0$ |     |     |     |     |     |     |     |
| `I` | $0$ |     |     |     |     |     |     |     |
| `G` | $0$ |     |     |     |     |     |     |     |
| `H` | $0$ |     |     |     |     |     |     |     |
| `A` | $0$ |     |     |     |     |     |     |     |
| `N` | $0$ |     |     |     |     |     |     |     |

</center>

Now we are going to start filling out our table, starting in the top left. Recall the general recursive procedure that we identified earlier:

1) If $A_n = B_m$ then we remove the last elements of $A$ and $B$ and increase our LCS length by one. We are finished and can return.
2) If $A_n$ and $B_m$ are different, we make three recursive calls:
i) A recursive call with the last element of $B$ removed ant $A$ left intact
ii) A recursive call with the last element of $A$ removed and $B$ left intact
iii) A recursive call with the last elements of $A$ and $B$ removed
3) Take the maximum of the three recursive calls and return.

In the top left corner, we have $A_n = $ `R` and $B_m = $ `K`. `R` and `K` are different of course, so we must make three recursive calls. Our table already has the values saved for these three calls! They are the three bordering table cells to the top left. If we want to think of these in terms of coordinates, for an entry `T[x][y]`, the values for our three recursive calls are in `T[x-1][y]`, `T[x][y-1]`, and `T[x-1][y-1]`. The maximum of three zeroes is, of course, zero, so we put zero in our table and pick a default direction, let's use $\uparrow$. The direction is used to indicate which element we removed from our sequence. $\uparrow$ indicates that we removed an element from our $B$ sequence (in this case `KERNIGHAN`). $\leftarrow$ indicates that we removed an element from our $A$ string (`RITCHIE`). $\nwarrow$ indicates that we removed an element from both sequences. This will become more clear shortly.


<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ |     |     |     |     |     |     |
| `E` | $0$ |     |     |     |     |     |     |     |
| `R` | $0$ |     |     |     |     |     |     |     |
| `N` | $0$ |     |     |     |     |     |     |     |
| `I` | $0$ |     |     |     |     |     |     |     |
| `G` | $0$ |     |     |     |     |     |     |     |
| `H` | $0$ |     |     |     |     |     |     |     |
| `A` | $0$ |     |     |     |     |     |     |     |
| `N` | $0$ |     |     |     |     |     |     |     |

</center>

Let's move down the first column. `E` and `R` don't match so we do the same thing as before. The max of all our recursive calls is zero, so we fill in a zero and our default direction ($\uparrow$). The next table cell is more interesting. `R` and `R` match! Referring back to our procedure, we want to remove an element from both sequences and increase the LCS length by 1. We point $\nwarrow$ (because we removed an element from both sequences) and set our LCS length to be $LCS(A_{1..2}, B_{1..1}) + 1$ (the value in the cell one cell $\nwarrow$ from the current cell). Here is what our table looks like:

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ |     |     |     |     |     |     |
| `E` | $0$ | $\uparrow 0$    |     |     |     |     |     |     |
| `R` | $0$ | $\nwarrow 1$    |     |     |     |     |     |     |
| `N` | $0$ |     |     |     |     |     |     |     |
| `I` | $0$ |     |     |     |     |     |     |     |
| `G` | $0$ |     |     |     |     |     |     |     |
| `H` | $0$ |     |     |     |     |     |     |     |
| `A` | $0$ |     |     |     |     |     |     |     |
| `N` | $0$ |     |     |     |     |     |     |     |

</center>

`N` and `R` don't match, so we make our three recursive calls. This time, the return values are $0$, $0$, $1$. The max of these is $1$ -- a tie no longer! This means that the LCS of `R` and `KERN` is $1$, which makes sense because `R` is in both sequences. But now what do we do with our direction? We want our direction to indicate which element we must remove to get the LCS with length $1$. We can't point $\nwarrow$$, because the LCS of ` ` and `KER` has length zero. We can't point $\leftarrow$ because the LCS of ` ` and `KERN` also has length zero. We always want to point to the table cell of the recursive call that returned the greatest length LCS. When we have a tie, it doesn't matter which one we point to (earlier when we had ties of $0$, $0$, and $0$ we just pointed $\uparrow$. We could have also pointed $\leftarrow$ or $\nwarrow$, it wouldn't have mattered. If we wanted to recover *all* LCSs of $A$ and $B$, then we would have to remember *all* the directions in case of a tie). The rest of this table row follows the same pattern; the $1$ gets passed all the way down (since after we pass the `R` in `KERNIGHAN`, we will always have a LCS of length one with `R`). This table looks like this:

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ |     |     |     |     |     |     |
| `E` | $0$ | $\uparrow 0$ |     |     |     |     |     |     |
| `R` | $0$ | $\nwarrow 1$ |     |     |     |     |     |     |
| `N` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `I` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `G` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `H` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `A` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `N` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |

</center>

Let's fill out the second column. We are now computing the LCS of `RI` with every truncated sequence of `KERNIGHAN`. `I` and `K` don't match, and neither do `I` and `E`. We have ties for zero on our recursive calls, so we have a LCS of length zero with direction $\uparrow$, as before. `I` and `R` don't match, but `RI` and `KER` do have a subsequence of length one, and this is captured in our recursive call to compute the LCS of `R` and `KER`. We mark down a length of one and point to the cell that gave us this value (in this case, $\leftarrow$).

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ | $\uparrow 0$   |     |     |     |     |     |
| `E` | $0$ | $\uparrow 0$ | $\uparrow 0$   |     |     |     |     |     |
| `R` | $0$ | $\nwarrow 1$ | $\leftarrow 1$ |     |     |     |     |     |
| `N` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `I` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `G` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `H` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `A` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `N` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |

</center>

Now for `I` and `N` we have a three way tie for our recursive calls. I will choose to continue to prefer pointing $\uparrow$ in the case of a tie. We keep the LCS length of one and point $\uparrow$. Next up we have `I` and `I`, a match! We take the LCS of $A$ and $B$ with the last elements removed, increment its length by one, and note that as our new LCS length. We point $\nwarrow$ to indicate that we removed the last element from both $A$ and $B$.

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ | $\uparrow 0$   |     |     |     |     |     |
| `E` | $0$ | $\uparrow 0$ | $\uparrow 0$   |     |     |     |     |     |
| `R` | $0$ | $\nwarrow 1$ | $\leftarrow 1$ |     |     |     |     |     |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 0$   |     |     |     |     |     |
| `I` | $0$ | $\uparrow 1$ | $\nwarrow 2$    |     |     |     |     |     |
| `G` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `H` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `A` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |
| `N` | $0$ | $\uparrow 1$ |     |     |     |     |     |     |

</center>

Great! The rest of this column and the next two columns follow suit with the previous ones. For the sake of brevity, I will fill out the table up to the `H` column. If you are reading this for understanding, I encourage you to first attempt filling out the table on your own and then checking your results with mine. Practice makes committing these algorithms to memory much easier!

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   |     |     |     |
| `E` | $0$ | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   |     |     |     |
| `R` | $0$ | $\nwarrow 1$ | $\leftarrow 1$ | $\leftarrow 1$  | $\leftarrow 1$ |     |     |     |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 1$   | $\uparrow 1$    | $\uparrow 1$   |     |     |     |
| `I` | $0$ | $\uparrow 1$ | $\nwarrow 2$   | $\leftarrow 2$  | $\leftarrow 2$ |     |     |     |
| `G` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   |     |     |     |
| `H` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   |     |     |     |
| `A` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   |     |     |     |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   |     |     |     |

</center>

The `H` column mostly follows suit with the previous two, until we get to the match at `H` and `H`. At this point, like previous matches, we point $\nwarrow$, take the value out of that cell, and increment it by one. Let's see what that looks like:

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   | $\uparrow 0$    |     |     |
| `E` | $0$ | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   | $\uparrow 0$    |     |     |
| `R` | $0$ | $\nwarrow 1$ | $\leftarrow 1$ | $\leftarrow 1$  | $\leftarrow 1$ | $\leftarrow 1$  |     |     |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 1$   | $\uparrow 1$    | $\uparrow 1$   | $\uparrow 1$    |     |     |
| `I` | $0$ | $\uparrow 1$ | $\nwarrow 2$   | $\leftarrow 2$  | $\leftarrow 2$ | $\leftarrow 2$  |     |     |
| `G` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 2$    |     |     |
| `H` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\nwarrow 3$    |     |     |
| `A` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 3$    |     |     |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 3$    |     |     |

</center>

The next column is a bit interesting because of the match with `I`. At this point we have a match so we point $\nwarrow$ and increment that value by one. But this means we have a tie with the cell one in the direction $\leftarrow$. This is because at this point we have two subsequences of equal length (two). We can take `RI` from `RITCHIE` in two different ways, using the first or the second `I`. We would have to account for this if we were attempting to enumerate all possible subsequences; but we are not, so we follow along with our normal rule of pointing $\nwarrow$.

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$    |     |
| `E` | $0$ | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$    |     |
| `R` | $0$ | $\nwarrow 1$ | $\leftarrow 1$ | $\leftarrow 1$  | $\leftarrow 1$ | $\leftarrow 1$  | $\leftarrow 1$  |     |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 1$   | $\uparrow 1$    | $\uparrow 1$   | $\uparrow 1$    | $\uparrow 1$    |     |
| `I` | $0$ | $\uparrow 1$ | $\nwarrow 2$   | $\leftarrow 2$  | $\leftarrow 2$ | $\leftarrow 2$  | $\nwarrow 2$    |     |
| `G` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$    |     |
| `H` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\nwarrow 3$    | $\leftarrow 3$  |     |
| `A` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 3$    | $\uparrow 3$    |     |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 3$    | $\uparrow 3$    |     |

</center>

The last column is filled out very similarly to the previous two. Try it for yourself first, if you are following along. Here is the completed table:

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | $0$ | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$    | $\uparrow 0$    |
| `E` | $0$ | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$    | $\nwarrow 1$    |
| `R` | $0$ | $\nwarrow 1$ | $\leftarrow 1$ | $\leftarrow 1$  | $\leftarrow 1$ | $\leftarrow 1$  | $\leftarrow 1$  | $\uparrow 1$    |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 1$   | $\uparrow 1$    | $\uparrow 1$   | $\uparrow 1$    | $\uparrow 1$    | $\uparrow 1$    |
| `I` | $0$ | $\uparrow 1$ | $\nwarrow 2$   | $\leftarrow 2$  | $\leftarrow 2$ | $\leftarrow 2$  | $\nwarrow 2$    | $\leftarrow 2$  |
| `G` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$    | $\uparrow 2$    |
| `H` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\nwarrow 3$    | $\leftarrow 3$  | $\leftarrow 3$  |
| `A` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 3$    | $\uparrow 3$    | $\uparrow 3$    |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 3$    | $\uparrow 3$    | $\uparrow 3$    |

</center>

We are finished! Now that we have our table, what is the length of our LCS? That is exactly $LCS(A, B)$, which is the call in the bottom right corner of our table. Thus the length of our LCS is three. If this is all the information we wanted to know (like in the case of Tingle Towers) then we are finished. However, because we marked down the direction we can also recover a LCS from our inputs. Starting in the bottom right corner, we follow the arrows until we get to a matching character. We note this down and keep following the arrows. Eventually we will get to the top row or the leftmost column and the characters that we have marked down form a LCS. Here is what this trace looks like:

<center class="lcs-table" style="overflow: auto;">

|     | ` ` | `R` | `I` | `T` | `C` | `H` | `I` | `E` |
|-----|-----|-----|-----|-----|-----|-----|-----|-----|
| ` ` | <span style="color: red">$0$</span> | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ | $0$ |
| `K` | <span style="color: red">$0$</span> | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$    | $\uparrow 0$    |
| `E` | <span style="color: red">$0$</span> | $\uparrow 0$ | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$   | $\uparrow 0$    | $\uparrow 0$    | $\nwarrow 1$    |
| `R` | $0$ | <span style="color: white; background-color: black;">$\nwarrow 1$</span> | $\leftarrow 1$ | $\leftarrow 1$  | $\leftarrow 1$ | $\leftarrow 1$  | $\leftarrow 1$  | $\uparrow 1$    |
| `N` | $0$ | <span style="color: red">$\uparrow 1$</span> | $\uparrow 1$   | $\uparrow 1$    | $\uparrow 1$   | $\uparrow 1$    | $\uparrow 1$    | $\uparrow 1$    |
| `I` | $0$ | $\uparrow 1$ | <span style="color: white; background-color: black;">$\nwarrow 2$</span>   | <span style="color: red">$\leftarrow 2$</span>  | <span style="color: red">$\leftarrow 2$</span> | $\leftarrow 2$  | $\nwarrow 2$    | $\leftarrow 2$  |
| `G` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | <span style="color: red">$\uparrow 2$</span>   | $\uparrow 2$    | $\uparrow 2$    | $\uparrow 2$    |
| `H` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | <span style="color: white; background-color: black;">$\nwarrow 3$</span>    | <span style="color: red">$\leftarrow 3$</span>  | <span style="color: red">$\leftarrow 3$</span>  |
| `A` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 3$    | $\uparrow 3$    | <span style="color: red">$\uparrow 3$</span>    |
| `N` | $0$ | $\uparrow 1$ | $\uparrow 2$   | $\uparrow 2$    | $\uparrow 2$   | $\uparrow 3$    | $\uparrow 3$    | <span style="color: red">$\uparrow 3$</span>    |

</center>

The path that we trace through the table is colored red, and the matching elements are highlighted. Looking at these highlighted cells, we know that a LCS of `KERNIGHAN` and `RITCHIE` is `RIH`! Filling out each cell in the table takes a constant amount of work and there are $n \times m$ cells in the table. Thus our dynamic programming algorithm is in $\mathcal{O} (nm)$, substantial savings over the $\mathcal{O}(2^n)$ cost of the naïve algorithm.

This algorithm is intuitive, fast, clever, and is a shining example of the power of dynamic programming. The LCS problem pops up all over the place in real world applications too, so this algorithm is a useful one to keep in your algorithmic toolbox. It also makes a killer Code-a-Thon problem! Don't worry, I still haven't forgotten.. without further ado, the $n$-dimensional generalization.

##### $n$-dimensional Generalization

To calculate a LCS of $n$ input sequences, we follow the exact same procedure that we did in two dimensions but with a slightly modified procedure. If we have a match for the last elements in all input sequences, we remove the the last elements of each sequence, take the LCS of those sequences, and increment by one. So in the case of a match of the last elements, we have $LCS(S_1, S_2, \dots , S_n)$ $=$ $LCS(S_{1, 1..l_1-1}, S_{2, 1..l_2-1}, \dots , S_{n, 1..l_n-1})$.

When the last elements do not match, we must make recursive calls to find the LCS for each possible combination of sequences with zero or one elements removed. For $n$ input sequences, there are $(2^n)-1$ recursive calls to make. For $n=2$ or $n=3$ this isn't that bad (3 and 7 calls, respectively), but for many inputs even our dynamic programming solution cannot solve this problem effectively. [Richard Bellmen](https://en.wikipedia.org/wiki/Richard_E._Bellman) (the inventor of dynamic programming) calls this the [curse of dimensionality](https://en.wikipedia.org/wiki/Curse_of_dimensionality).

Fortunately, for three input strings this problem is still computationally feasible. We must make our table three dimensional. Instead of the three directions from before, we now have seven. I notate these by the combinations of axes in three dimensional space that each direction points along. We have `X` `Y` and `Z` for when the direction goes straight along each axis, `XY` for when the direction points at a $\frac{\pi}{4}$ angle between `X` and `Y`, and `XZ` and `YZ` with similar explanations. Finally, the XYZ direction is the equivalent to our $\nwarrow$ direction in the two dimensional example above and indicates the removal of the last element for each of our input strings. It's difficult to visualize this 3D table in your head (and even harder on paper). For four or more dimensions this visualization becomes impossible, although we can still utilize the algorithm using the exact same approach, keeping an $n$-dimensional array as our table.

##### Code Solution

The full Python code for my solution is below:

```python
# We grab our three lines of input and split them by character.
line1 = list(input())
line2 = list(input())
line3 = list(input())

class Entry:
    # By default, an entry has no set direction and a value of 0.
    def __init__(self):
        self.direction = Direction.UNSET
        self.val = 0 

# The direction scheme here points to an adjacent entry in the table.
# For example, XZ implies the direction back one x coordinate and back one
# z coordinate.
class Direction(Enum):
    X     = 0
    Y     = 1
    Z     = 2
    XY    = 3
    XZ    = 4
    YZ    = 5
    XYZ   = 6
    UNSET = 7

# Our table will have x_max * y_max * z_max entries in it.
x_max = len(line3)
y_max = len(line2)
z_max = len(line1)

# We build our table and fill it with empty entries (value = 0 and direction UNSET)
table = []
for i in range(x_max + 1):
    a = []
    for _ in range(y_max + 1):
        b = []
        for _ in range(z_max + 1):
            b.append(Entry())
        a.append(b)
    table.append(a)

# We construct our table used in memoization by iterating over every entry.
for x in range(1, x_max + 1):
    for y in range(1, y_max + 1):
        for z in range(1, z_max + 1):
            # If we have a match on the x,y, and z coordinates then we increment
            # the value for that table entry and set the dirextion to point to
            # the previous value.
            if line3[x-1] == line2[y-1] and line2[y-1] == line1[z-1]:
                table[x][y][z].val = table[x-1][y-1][z-1].val + 1
                table[x][y][z].direction = Direction.XYZ
            else:
                # Otherwise, we find the highest value neighbor, copy its value,
                # and point to it.
                max_ind, max_val = max(enumerate([
                    table[x-1][y][z].val,
                    table[x][y-1][z].val,
                    table[x][y][z-1].val,
                    table[x-1][y-1][z].val,
                    table[x-1][y][z-1].val,
                    table[x][y-1][z-1].val,
                    ]), key=lambda p: p[1])
                table[x][y][z].val = max_val
                table[x][y][z].direction = Direction(max_ind)

longest = []

print(table[x_max][y_max][z_max].val)

# Now that our table is filled out, we can trace back from the last entry and
# reconstruct a longest subsequence. The commented code below prints out a
# longest common subsequence.
# current_x = x_max
# current_y = y_max
# current_z = z_max
# while table[current_x][current_y][current_z].val is not 0:
    # current_direction = table[current_x][current_y][current_z].direction
    # if current_direction == Direction.XYZ:
        # longest.append(line3[current_x - 1])
        # current_x = current_x - 1
        # current_y = current_y - 1
        # current_z = current_z - 1
    # elif current_direction == Direction.X:
        # current_x = current_x - 1
    # elif current_direction == Direction.Y:
        # current_y = current_y - 1
    # elif current_direction == Direction.Z:
        # current_z = current_z - 1
    # elif current_direction == Direction.XY:
        # current_x = current_x - 1
        # current_y = current_y - 1
    # elif current_direction == Direction.XZ:
        # current_x = current_x - 1
        # current_z = current_z - 1
    # elif current_direction == Direction.YZ:
        # current_y = current_y - 1
        # current_z = current_z - 1
# 
# longest.reverse()
# print(''.join(longest))
```

The input for this problem was generated with the following Python script:

```python
import random

coll = ['r', 'b', 'g']

(x, y, z) = list(map(lambda x: x + 1, map(int, input().split(' '))))

xs = [''] * x
for i in range(x):
    xs[i] = random.choice(coll)
print(''.join(xs))

ys = [''] * y
for i in range(y):
    ys[i] = random.choice(coll)
print(''.join(ys))

zs = [''] * z
for i in range(z):
    zs[i] = random.choice(coll)
print(''.join(zs))
```

Input numbers for the lengths of each sequence (`x`, `y`, and `z` above) were fed in from another script, which then called the generator above to create the tests cases.

#### Problem 5: Team Rocket's Capture Caper

This problem fulfills another USC Code-a-Thon tradition: using Team Rocket as the backstory for a problem. Usually our Team Rocket problems are some variant on graph search (DFS, BFS, or Dijskstra's). I decided to change it up this year and do a problem type that we have never done before. This problem was one I hadn't actually solved before, so it was fun to implement the solution for the first time.

In this problem, Team Rocket is capturing Pokemon in a field using a net. The Pokemon are scattered all over the field, and we are given the x and y coordinates of each one. We are asked to find the minimum area of some net that Team Rocket needs to capture all the Pokemon in one go (meaning the net must be big enough to cover every Pokemon in the field). There is a catch though, the net must be a [convex polygon](https://en.wikipedia.org/wiki/Convex_polygon): if we were to imagine a line from any corner of the net to any other corner of the net, this line must be entirely contained within the polygon. I actually forgot to specify this restriction in the problem description at first (oops!). I was able to clarify this pretty quickly after someone asked me about it with an example. Fortunately, most students assumed that this restriction was in place, as the main component of this question is a famous problem in computer science.

This problem has two components to it:
1) Finding the points on the grid that make up the net
2) Finding the area of the net

The first component is where the bulk of the work takes place, and is the primary focus of this problem. We need to find a convex polygon that covers all the points in a set of points. When I Google "convex polygon that covers a set of points" the fourth result is the Wikipedia page for [Convex Hull](https://en.wikipedia.org/wiki/Convex_hull). Sure enough, the convex hull problem is the problem we are attempting to solve.

There are [many algorithms](https://en.wikipedia.org/wiki/Convex_hull_algorithms#Algorithms) that can be used to calculate the Convex Hull of a set of points. I opted to use [Graham's Scan](https://en.wikipedia.org/wiki/Graham_scan) for its intuitiveness and for its presence in CLRS. I will describe Graham's Scan, and then a method for calculating the area of a convex polygon.

##### Graham's Scan

I love algorithm visualizations. If a picture is worth a thousand words, then a visualization is worth a million. Wikipedia user Shiyu Ji has created an excellent animation showing the operation of Graham's Scan. Take a moment to watch this animation, and then I will explain in detail what is going on.

<center>
    <img style="width: 100%; max-width: 344px" src="/assets/blog/spring-2018-code-a-thon-solutions/graham_scan_animation.gif" width="100%" alt="Animation showing the operation of graham's scan">
</center>

The first thing we must do is select a starting point from which to build our convex hull. This starting point should be the lowest point in our set (the point with the lowest y-coordinate). In the case of a tie, we want to select the leftmost point (so we minimize the y-coordinate first, then we minimize the x-coordinate). This point can  be found in $\mathcal{O}(n)$ time where $n$ is the number of points in our set. Note that this point *must* be a point in our convex hull, since no other point is lower (and thus would be able to provide coverage over this point). This initial point we will call $p_0$. Next we must sort our remaining points in order of increasing polar angle with respect to $p_0$. Picture a radar system with a scan line rotating counter clockwise. As the scan line rotates around the origin, it highlights points on the radar. If our origin is point $p_0$ and the points that get highlighted on the radar are the remaining points in our data set, then the order in which the points get scanned by the radar is the order in which we want our points to be sorted.

We are using a [polar coordinate system](https://en.wikipedia.org/wiki/Polar_coordinate_system) with $p_0$ as our origin. Given the x and y coordinates of a point, we can easily calcuate the angle it forms with the origin by using the [$atan2$](https://en.wikipedia.org/wiki/Atan2) function present in most programming languages. $atan2(y, x)$ will return the angle that $(x,y)$ forms with the point $(0,0)$. We want to know what angle $(x,y)$ forms with $p_0$, however, so we must [translate](https://en.wikipedia.org/wiki/Translation_(geometry)) our set by the vector that points to $p_0$. This is pretty easy to do; if $p_0 = (x', y')$ then we just need to subtract $(x', y')$ from every other point. This can be done in our for loop while we calculate the angles. Assuming we have our $p_0$ stored in a variable `p0` and the rest of our points stored in some variable `points` our pseudocode would look like this:

```
for point in points
    point.angle = atan2(point.y - p0.y, point.x - p0.x)
```

Then we simply sort `points` by the `angle` property on each point, and we have our list of points in counterclockwise order around $p_0$! There are two extra optimizations we can do here:

1) If we have three points (let's say $p_0, p_1, p_2$, with $p_0$ being our origin point as above) such that all three points are [colinear](https://en.wikipedia.org/wiki/Collinearity), we don't need to keep the middle point on the line in our set of points. This is because any convex hull that includes the two outer points on the line would also include the inner point. Because $y \geq y'$ for all points compared to $p_0$, for any two points with the same polar angle with respect to $p_0$ we can simply forget the one with lower y coordinate.

2) $atan2$ is a more expensive operation than we need to perform. In fact, we can simply use the slope of the line that is formed between $p_0$ and every other point. This requires some extra logic (slope increases from 0 to $\infty$ until we cross the y-axis and start getting negative values from $-\infty$ to 0) but should perform slightly faster since we are only doing a few basic integer arithmetic operations. However, this does not change the algorithmic complexity of the algorithm, so I chose to omit this optimization from my solution.

Once we have our points sorted in counterclockwise order, we initialize a [stack](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)) and push the first three points from our sorted list (starting with $p_0$) onto the stack. Note that our stack must implement a $Peek$ function to return the value of the top element of the stack (without removing it from the stack) as well as a $Peek2$ function which returns the value of the second element in our stack (again without removing it from the stack). Now we iterate over the remaining points in our sorted list and perform the following procedure:

0) The point we are considering in our current iteration of our for loop is stored in a variable `point`. The top of our stack is returned by the function `stack.peek()`. The second to top element of our stack is returned by the function `stack.peek2()`.
1) We consider the angle that is made between `stack.peek2()`, `stack.peek()`, and `point`.
2) If this angle is a **left turn** then we push `point` onto our stack and contiune to the next point in our for loop.
3) If this angle is a **right turn** then we pop the top value off of our stack and go back to step 1 repeating this, until we get a left turn.

In pseudocode, this might be implemented like the following:

```
for point in points
    while angle_between(stack.peek2(), stack.peek(), point) is right_turn
        stack.pop()
    stack.push(point)
```

You should be asking yourself, "how do we define a left turn or right turn?" I'm glad you asked. If we picture a car driving from `stack.peek2()` to `stack.peek()` and then to `point`, then we are interested in the direction that the driver turns her wheel when she gets to `stack.peek()`. If she turns the wheel left, this is a left turn. If she turns the wheel right, this is a right turn. It's a fairly intuitive concept, but it's less intuitive to calculate. Similarly to before, we don't actually need to calcuate the angle. We instead use the geometric properties of the [cross product](https://en.wikipedia.org/wiki/Cross_product) to figure out the turn direction. If $p_1 = (x_1, y_1)$, $p_2 = (x_2, y_2)$, $p_3 = (x_3, y_3)$, then algebraically we can figure out the turn direction using the following formula:

$$D = (x_2 - x_1)(y_3 - y_1) - (y_2 - y_1)(x_3 - x_1)$$

If $D$ is positive then we have a left turn. If $D$ is negative then we have a right turn. If $D$ is 0 then the three points are colinear and we can treat it as a left turn (pushing our point onto the stack and continuing).

After reading this, rewatch the above animation. The points connected by the red line segments are the points in the stack; the last two points connected by a red line segment are the top two points in the stack. The point connected by the blue line segment is the point that we are considering in the current iteration of our for loop. The animation does a great job of showing how the convex hull is constructed using this algorithm.

When we finish the procedure above for all the points in our set, the points in the stack are exactly the points that form the convex hull, in counterclockwise order. So how long does Graham's scan take? The actual convex hull calculating time in the procedure we described above runs in $\mathcal{O}(n)$ time; this result is perhaps somewhat surprising, but each point is only considered at most a constant number of times, so this step runs in linear time on the number of points. However, we had to sort our points earlier by their relative polar angles with our initial point. Sorting our set of points takes $\mathcal{O}(n \lg n)$ time, which dominates the overall runtime of our algorithm. Thus Graham's scan is in $\mathcal{O}(n \lg n)$ where $n$ is the number of points for which we are finding the convex hull.

After the application of Graham's scan, we now have the points of our convex hull, convienently sorted in counterclockwise order. The bulk of our work is complete, but we must still find the area of our net in order to return an answer. Fortunately, this is fairly easy to compute.

##### Area of a Convex Polygon

A brief story: on my search for a convienent algorithm for convex polygon area calculation I stumbled upon [this Math for Dummies](http://www.dummies.com/education/math/algebra/finding-the-area-of-a-triangle-using-its-coordinates/) post on how to find the area of a triangle using its coordinates. Below is a screenshot of the article that I linked:

<center>
    <img src="/assets/blog/spring-2018-code-a-thon-solutions/dummies.png" style="width: 100%; max-width: 725px;" alt="Dummies article">
</center>

If we use our classic $A = \frac{1}{2} \cdot base \cdot height$, we can see that the area of this triangle is 25 square units, not 50. The author forgot to divide by two in their calculation! Naturally, I submitted an eratta report to Wiley so that they may fix this error. On March 20th I received the following e-mail in response:

<center>
    <img src="/assets/blog/spring-2018-code-a-thon-solutions/dummies_email.png" style="width: 100%; max-width: 779px;" alt="Dummies article">
</center>

A little less than a month later, I received this e-mail:

<center>
    <img src="/assets/blog/spring-2018-code-a-thon-solutions/dummies_email_2.png" style="width: 100%; max-width: 779px;" alt="Dummies article">
</center>

Looks like they have acknowledged the error! As of writing this (April 25th) the article in question still has not been updated, but I have hopes that it will get fixed soon. Hopefully before some poor geometry student fails a homework!

Anyway, the algorithm for finding the area of a convex polygon is quite simple, especially if the points of the polygon are already sorted in counterclockwise order (like they are after Graham's scan). It wasn't until after the Code-a-Thon that I learned that there is a well known name for the method that I employed: the [shoelace algorithm](https://en.wikipedia.org/wiki/Shoelace_formula). 

Essentially, the way the shoelace algorithm works is by breaking the polygon into triangles, calculating the area of the rectangle around each triangle, and then subtracting the areas of each region in the rectangle that is not part of the triangle (of which there are exactly three). Wikipedia provides a good explanation of how this works in the [proofs section](https://en.wikipedia.org/wiki/Shoelace_formula#Proofs) of the page linked above.

The formula for this area calculation looks like this:

$$A = \frac{1}{2} | \sum_{i=1}^{n} x_{i}y_{i+1} - x_{i+1}y_{i}|$$

where $n$ is the number of points we have and $p_i = (x_i, y_i)$. Note that the above formula will reference the point $(x_{i+1}, y_{i+1})$. We desire this overflow point to loop back to the first point, so $(x_{i+1}, y_{i+1}) = (x_1, y_1)$.

Implementing this is pretty straightforward:

```
area = 0
for index in range(0, points.length - 1)
    p0 = points[index]
    p1 = points[index+1]
    area = area + p0.x * p1.y - p1.x * p0.y
first = points[0]
last = points[points.length - 1]
area = area + last.x * first.y - first.x * last.y
area = area / 2
```

We iterate through every point and perform our computation with it and the next point. When we get to the end, we have a special case to handle the last multiplication that will wrap around our list of points. We divide our running total by a factor of 2 (this is where Wiley messed up) and then we have our area! We can print out this value and we have our answer.

##### Discussion

I explicitly disabled the [Octave](https://www.gnu.org/software/octave/) programming language for this problem, because it has a built-in [convex hull function](https://octave.org/doc/v4.2.1/Convex-Hull.html). However, I did not disable the [R](https://www.r-project.org/) programming language, which also has a built-in convex hull function. One participant noticed this and exploited it to get a working solution. This is perhaps unfortunate, but a good programmer should always exploit available tools, and I salute this student for their ingenuity. Three other students found code that solved this problem somewhere online and exercised their Ctrl+C Ctrl+V muscles to solve this problem. This is impossible for us to police, so I only hope that these students took the time to understand the code that they were copying. I do know that one student (the first place winner at that!) took the time and effort to implement a solution manually. Props to this student! If they ever need to implement a convex hull algorithm again, they will be the best prepared for this feat.

##### Code Solution

The full Python code for my solution is below:

```python
import math

n = int(input())
points = [[0, 0, 0]] * n

for i in range(n):
    points[i] = list(map(float, input().split(' '))) + [0]

# Sort by x and then y
points.sort(key=lambda x: x[0])
points.sort(key=lambda x: x[1])

# Remove the first point, which is the lowest leftmost points. This point is
# part of our convex hull.
p0 = points[0]
points = points[1:]

# Now we determine the polar coordinates of each point relative to p0
# We don't actually have to compute the angle here, but this is still O(n)
# so we won't worry about it for now.
for point in points:
    point[2] = math.atan2(point[1] - p0[1], point[0] - p0[0])

# We sort by the polar angles to make a stack of increasing polar coordinates
# relative to p0
points.sort(key = lambda x: x[2])

# If two points have the same radial angle from p0, the point closer to p0
# (the one with smaller x and y coordiates) can be removed as it cannot be
# part of our convex hull. This reduces the number of points we need to process
# later.
removal_indices = []
for i in range(len(points) - 1):
    if points[i][2] == points[i+1][2]:
        removal_indices.append(i)

for index in reversed(removal_indices):
    points.pop(index)

# We perform Graham's scan on the points to determine the convex hull.
stack = [p0, points[0], points[1]]
for i in range(2, len(points)):
    p3 = points[i]
    cross = -1
    while cross < 0:
        p1 = stack[-2]
        p2 = stack[-1]
        # This quickly calculates the cross product between p1p2 and p1p3.
        # If `cross` is 0, the points are colinear. If `cross` is positive we
        # have made a left turn and can move on to the next point. If `cross` is
        # negative, we have made a right turn and pop the top point and
        # continue.
        cross = (p2[0] - p1[0]) * (p3[1] - p1[1]) - (p2[1] - p1[1]) * (p3[0] - p1[0])
        if cross < 0:
            stack.pop()
    stack.append(points[i])

# Now `stack` contains the points of our convex hull in counterclockwise order.
# We can now calculate the area of the polygon.
# This is the de-facto way of computing area for a 2D convex polygon, and is
# an application of Green's theorem.
area = 0
for i in range(1, len(stack)):
    area += stack[i-1][0]*stack[i][1] - stack[i][0]*stack[i-1][1]
# Account for the wrapping from the last point to the first...
area += stack[-1][0]*stack[0][1] - stack[0][0]*stack[-1][1]
area = area / 2

print(area)
```

Several students ran into an issue in their area calcuation code where they did not wrap around to the first point. This is an error I made while coding my initial solution, so I sympathized when people showed me *almost* working code that forgot to account for that last edge in their polygons. That's not a mistake those students will make again!

#### Problem 6: Zucchini Zipping

This was, based on the questions I received from participants, the hardest problem to understand. There is a reason that my explanation was so... strange. This question was inspired by (based on... or actually ripped off of) a problem from the 2014 ACM-ICPC southeastern region qualifier. After reading my shoddy explanation for my version of the problem, read problem H of [that year's problem set](http://serjudging.vanb.org/wp-content/uploads/SER-2014-D1-Problems.pdf). Make more sense? Card shuffling is a very natural way to represent this problem, but I avoided using that analogy because I didn't want people to be able to Google for it and find the original problem. Thus "zucchini zipping" was born! I tried my best...

This problem was the hardest problem in this semester's problem set. Only two students solved it (the two that solved every problem). This is because this was the only problem that required some level of personal ingenuity. Not to say that the other problems were easy; they weren't. But the other hard problems in our problem set were testing students' algorithms knowledge and their ability to implement these algorithms. Pricey Power was the minimum spanning tree problem; Tingle Towers was the longest common subsequence problem; Team Rocket's Capture Caper was the convex hull problem. But this problem wasn't a wrapper around some other famous computer science problem: it was a problem that required a custom algorithm to solve a unique problem.

##### The Problem

I will use the card shuffling analogy that was used in ICPC, as it is much easier to understand. Picture a deck of $N$ cards, labeled from $1$ to $N$. We split the deck in two by taking $M$ cards off the top. We now have two stacks of cards, one with $M$ cards and one with $N-M$ cards. We now take some arbitrary number of cards off the bottom of the first stack of cards and add it to a third stack. Then we take an arbitrary number of cards from the second stack and add them onto the third stack. We alternate back and forth between the two stacks until all cards have been moved into the third stack. Let's walk through an example:

We have the cards one through eight in a stack:

$$
1, 2, 3, 4, 5, 6, 7, 8
$$

We split this stack into two stacks. We pick an arbitrary location to perform the split: for this exmaple let's have a stack from $1$ to $5$ and a stack from $6$ to $8$.

$$
1, 2, 3, 4, 5
$$

$$
6, 7, 8
$$

Now we take some arbitrary number of cards from the bottom of the first stack and add it to a new third stack. Let's take $1$ and $2$. Now we have:

$$
3, 4, 5
$$

$$
6, 7, 8
$$

$$
1, 2
$$

Now let's take only one card from the second stack ($6$):

$$
3, 4, 5
$$

$$
7, 8
$$

$$
1, 2, 6
$$
 
Back to the first stack. Let's take two more.

$$
5
$$

$$
7, 8
$$

$$
1, 2, 6, 3, 4
$$

Let's now take the rest of the second stack:

$$
5
$$

$$
1, 2, 6, 3, 4, 7, 8
$$

We have only one option at this point. We must take the rest of the first stack, since our second stack is empty. Our once-shuffled stack now looks like this:

$$
1, 2, 6, 3, 4, 7, 8, 5
$$

We can repeat this operation. After shuffling the above (once-shuffled) card sequence, we can end up with many different combinations. Here are some examples:

$$
6, 3, 4, 7, 8, 5, 1, 2
$$

$$
1, 3, 4, 7, 8, 2, 5, 6
$$

$$
1, 2, 3, 4, 7, 8, 5, 6
$$

That last example is interesting... let's start from a fresh sequence and do another example:

$$
1, 2, 3, 4, 5, 6, 7, 8
$$

Split between $6$ and $7$,

$$
1, 2, 3, 4, 5, 6
$$
$$
7, 8
$$

We will take the first four from the first stack, then the entire second stack, and then the remainder of the first stack. We are left with:

$$
1, 2, 3, 4, 7, 8, 5, 6
$$

That's the same sequence that we got in our third example above! So this sequence can be reached using both one and two shuffles. This is important, because we are only interested in the **minimum** number of times a sequence has been shuffled. For the above sequence, the answer is one.

##### Analysis

Before attempting to be clever, it's often a good idea to at least consider a brute force solution. Even if the resulting solution is too slow, it usually enhances one's intuition about the problem. How would we go about brute forcing this problem? We know the starting point (cards numbered from $1$ to $N$) and ending point (it's given to us in the input). We know that we are looking for the minimum number of shuffles needed from the starting point to the ending point. So if we consider all possible sequences generated with one shuffle, then with two shuffles, three, etc., as soon as we find a match we return the iteration we are currently on (one shuffle, two shuffle, etc.). For our analysis, we need to know how many possible ways there are to shuffle $N$ cards. Let's consider the case with four cards.

$$
1, 2, 3, 4
$$

There are five places we can split this sequence: `| 1 | 2 | 3 | 4 |`. If we split the sequence at the first or last location, then we always get the same sequence as before. If we split between the `1` and the `2`, then we can get `1 2 3 4`, `2 1 3 4`, `2 3 1 4`, or `2 3 4 1`. If we split between the `2` and the `3`, we can get `1 3 4 2`, `1 3 2 4`, `1 2 3 4`, `3 1 2 4`, `3 4 1 2`, or `3 1 4 2`. If the split between the `3` and the `4`,  then we can get `4 1 2 3`, `1 4 2 3`, `1 2 4 3`, or `1 2 3 4`. Thus there are 16 ways to shuffle four elements once. Then for each of our 16 results, we will have 16 more results. By the time we have reached our fourth shuffle, we will have checked over 65 thousand sequences; that level of growth is not promising. You may have noticed that we are doing lots of extra work. Even in the 16 shuffles that I gave above, there were duplicates. We don't ever need to consider a sequence more than once, since the first time we find our sequence will be the minimum number of shuffles we have. This seems like it could be a good application for dynamic programming. We make a table for every possible sequence and fill it out until we have found our desired sequence. We will need a table entry for every possible sequence. Since our original sequence has $n$ distinct elements, there are $n!$ many [permutations](https://en.wikipedia.org/wiki/Permutation) of the sequence. The input constraint gives us an  upper bound of $n=2^{20}$. Unfortunately there is not enough matter in the universe to keep a table of $2^{20}!$ sequences in memory, so there must be a better way.

Let's think about what happens when we perform a shuffle. We split the deck into two sequences from $1$ to $k$ and from $k+1$ to $n$. These sequences are then shuffled together, but the [subsequences](https://en.wikipedia.org/wiki/Subsequence) are preserved in both. Thus we now have 2 contiguous subsequences of our original deck. When I say contiguous subsequence of our original deck, I mean that we now have some subsequence of our shuffled deck that was a contiguous subsequence (or [substring](https://en.wikipedia.org/wiki/Substring)) of our original deck. Thus `2 3 4` could be a contiguous subsequence of our original deck, but `2 3 5` could not be. Now what happens when we shuffle again? Let's look at an example, slightly longer this time.

$$
1, 2, 3, 4, 5, 6, 7, 8
$$

Split in the middle...

$$
1, 2, 3, 4
$$
$$
5, 6, 7, 8
$$

And then shuffled together,

$$
1, 5, 2, 6, 3, 7, 4, 8
$$

Notice we have two contiguous subsequences of our original deck: `1 2 3 4` and `5 6 7 8`. Now let's shuffle again:

$$
1, 5, 2, 6, 3, 7, 4, 8
$$

Split in the middle...

$$
1, 5, 2, 6
$$
$$
3, 7, 4, 8
$$

And then shuffled together,

$$
1, 3, 5, 7, 2, 4, 6, 8
$$

We now have four contiguous subsequences of our original deck: `1 2`,  `3 4`, `5 6`, `7 8`. Notice in the worst case, we can have eight contiguous subsequences of our original deck. This would be when we have `8 7 6 5 4 3 2 1`. How many shuffles does it take to get here? Let's try it out.

$$
1, 2, 3, 4 | 5, 6, 7, 8
$$

to

$$
5, 1, 6, 2 | 7, 3, 8, 4
$$

to

$$
7, 5, 3, 1 | 8, 6, 4, 2
$$

to

$$
8, 7, 6, 5, 4, 3, 2, 1
$$

So it takes three shuffles. This means in the worst case, three shuffles will produce eight different contiguous subsequences of our original deck. Hmm... one shuffle makes two subsequences, two shuffles makes four subsequences, and three shuffles makes eight subsequences. There's a pattern here! If we have $n$ contiguous subsequences of our original deck present in our shuffled deck, then we must have shuffled our original deck a minimum of $\log_{2} n$ times. What if we have a number of subsequences that is not a power of two? That's okay; if we have two subsequences it means our deck was shuffled once. We can never get three subsequences from shuffling once. If we have four subsequences, then our deck was shuffled twice. So three subsequences is obtainable from two shuffles, but not one. We can take the ceiling of our log operation to get the definitive answer.

This is a great move in the right direction. We now have a way to relate the number of shuffles that were performed to the number of contiguous subsequences of our original deck that are present in our shuffled deck. Now we need a way to find how many contiguous subsequences of our original deck there are in our shuffled deck.

##### $\mathcal{O}(n^2)$ solution

Let's consider the twice shuffled deck from our previous example:

$$
7, 5, 3, 1, 8, 6, 4, 2
$$

We know the four contiguous subsequences of our original deck are `7 8`, `5 6`, `3 4`, and `1 2`. Looking at this, it seems like we can start at the left of our sequence and scan through the sequence looking for the next contiguous value. Every time we find this next value in our sequence, we remove it from the sequence. The number of times we scan through our array is exactly the number of contiguous subsequences of our original deck that we have. Let's see what this would look like:

We start with `7 5 3 1 8 6 4 2`. We start with  `7` and then scan  through our  sequence looking for an `8`. We find the `8` and then keep scanning for a `9`. Since there is no `9` in our sequence, we reach the end of the array and remove the `7` and `8`. Now we are left with `5 3 1 6 4 2`. We start with `5` and scan through our sequence looking for `6`. We find the `6` and then keep scanning for a `7`. We do not find the `7` as it was already removed, so when we reach the end of the array we take out `5` ant `6`. We have now gone through the array twice. We have `3 1 4 2`. Startinig with `3`, we will scan until we find the `4` and then reach the end. We are now left with `1 2`. We do one last scan, remove `1` and `2` and are left with an empty sequence. We scanned through the array 4 times, so we have 4 contiguous subsequences of our original deck which means this deck must have been shuffled twice ($\log_{2} 4 = 2$).

We now have a solution! Let's look at its worst case performance. We will consider the case `8 7 6 5 4 3 2 1`. Looking at this, it seems we will have to scan through our array 8 times, once for each subsequence. In general, for a sequence of $n$ numbers we will have to scan through the array $n$ times in the worst case. This means our solution is in $\mathcal{O} (n^2)$. Much better than checking each of the $n!$ permutations. But can we do better?

##### $\mathcal{O}(n)$ solution

The answer to that question (as it usually is) is yes. Since we need to know how many subsequences of our original deck we have, it seems likely that we will have to check every element in the sequence at least once. $\mathcal{O} (n^2)$ is too slow but it seems unlikely we can go sublinear. Thus, let's look for a linear solution. For a linear solution, we need a way to find the number of subsequences in our sequence using only a constant number of passes over our array. Let's consider some element of our sequence $k$. We know that $k$ is between $1$ and $n$ where $n$ is the number of elements in our sequence. $k$ is part of some subsequence in our original deck. The preceeding element in this subsequence is $k-1$ and the next element in our subsequence is $k+1$ (unless $k=1$ or $k=n$; we will not consider these cases for the sake of generalization). If $k-1$ is in our shuffled deck before $k$, then $k$ continues the subsequence in which $k-1$ was the previous element. If $k+1$ is in the shuffled deck after $k$, then $k+1$ contiunes the subsequence in which $k$ is its previous element. We want to know the total number of subsequences of our original deck, so we can simply find the number of elements that start subsequences of our original deck (since each subsequence has exactly one starting element). What is a starting element of a subsequence? It is some element $k$ where $k-1$ does not come before $k$ in the shuffled deck. Conversley, an element $k$ is *not* the first element of some subsequence if $k-1$ has come before it in the deck. This seems promising...

Let's iterate through our shuffled deck. We start with the first element, we will call it $k$. Of course, it is impossible to have seen $k-1$ earlier in the deck, so $k$ must be the first element of some subsequence of our original deck. We increment some `SequencesFound` counter. We also want to somehow let element $k+1$ know that it is not the first element of a subsequence, since $k$ came before it in the deck. We can't do a linear time operation here, because then our final solution would be in $\mathcal{O} (n^2)$, so scanning for element $k$ is out. We need some kind of constant time operation. Amongst the most common constant time operations are array accesses in which index to access is already known (or can be computed in constant time). We don't know the index of element $k+1$, but we do know its value (it's $k+1$). We also know that every element in our deck is unique and that we have exactly $n$ elements. So we can index into a *different* array using index $k+1$. We set the value of some other array at index $k+1$ to `True`, to indicate that element $k+1$ is not the first element of any subsequence. We continue our iterating through the array... fast forward until we are considering element $k+1$. We check our secontary array at index $k+1$ and find the value `True`. This indicates to us that we have already found element $k$ previously in our deck, so we do not increment our subsequence counter. We do, however, set index $k+2$ in our secondary array to `True` to indicate that $k+2$ is not the starting element of some subsequence.

Let's walk through an example.

Starting with the following deck:

$$
1, 2, 3, 4, 5, 6, 7, 8
$$

we shuffle once to obtain

$$
4, 5, 1, 6, 2, 3, 7, 8
$$

and then shuffle again to obtain

$$
8, 4, 5, 1, 6, 2, 3, 7
$$

<center class="lcs-table" style="overflow: auto;">

Sequences Found = 0

Shuffled Deck Array

| index | <span style="color: red">1</span> | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| value | <span style="color: red">8</span> | 4 | 5 | 1 | 6 | 2 | 3 | 7 |

Secondary Array

| index | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| value | `False` | `False` | `False` | `False` | `False` | `False` | `False` | `False` |

</center>

We start with our first element, $8$. We check our other array at index $8$ and find `False`. We increment our sequences found variable and mark index $9$ in our other array as `True`. There is no index $9$, so we do nothing. Now we consider our second element, $4$. We check our other array at index $4$ and find `False`. We increment our sequences found variable and mark index $5$ as true in our table. Here's the current state of our algorithm:

<center class="lcs-table" style="overflow: auto;">

Sequences Found = 2

Shuffled Deck Array

| index | 1 | 2 | <span style="color: red;">3</span> | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| value | 8 | 4 | <span style="color: red;">5</span> | 1 | 6 | 2 | 3 | 7 |

Secondary Array

| index | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| value | `False` | `False` | `False` | `False` | <span style="color: red">`True`</span> | `False` | `False` | `False` |

</center>

Now we consider our third element, $5$. We check index $5$ in our other array, and find that we have a `True`. This means that element $4$ has appeared previously in our shuffled deck, so $5$ can not be the first element of some subsequence. We do not increment our sequences found counter, but we do mark index $6$ in our other array as `True`.

<center class="lcs-table" style="overflow: auto;">

Sequences Found = 2

Shuffled Deck Array

| index | 1 | 2 | 3 | <span style="color: red;">4</span> | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| value | 8 | 4 | 5 | <span style="color: red;">1</span> | 6 | 2 | 3 | 7 |

Secondary Array

| index | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| value | `False` | `False` | `False` | `False` | `True` | <span style="color: red;">`True`</span> | `False` | `False` |

</center>

Onto the fourth element. We have $1$ and we check it in our other array. `False`, so we increment our sequences found counter and mark $2$ as `True`. Next up is $6$. We look this up in our other array and find `True`. We mark $7$ as `True`. Then we have $2$. We look this up, find `True`, and then mark $3$ as `True`. Here's what we have now:

<center class="lcs-table" style="overflow: auto;">

Sequences Found = 3

Shuffled Deck Array

| index | 1 | 2 | 3 | 4 | 5 | 6 | <span style="color: red;">7</span> | 8 |
|---|---|---|---|---|---|---|---|---|---|
| value | 8 | 4 | 5 | 1 | 6 | 2 | <span style="color: red;">3</span> | 7 |

Secondary Array

| index | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| value | `False` | <span style="color: red;">`True`</span> | <span style="color: red;">`True`</span> | `False` | `True` | `True` | <span style="color: red;">`True`</span> | `False` |

</center>

Only two more. We see $3$, note that it is `True` in the other way, and then mark $4$ as `True`. Then we reach $7$, note that it is `True`, and mark $8$ as `True`. We have reached the end of our array! We take $\log_2$ of the number of sequences ($3$) and then take the [ceiling](https://en.wikipedia.org/wiki/Floor_and_ceiling_functions). $\lceil \log_2(3) \rceil = 2$, which is the number of shuffles that we performed. We only scanned through the array once and performed a constant amount of work for each array element, which means we got an answer in linear time! Awesome!

##### Code Solution

The full Python code for my solution is below:

```
# Read in number of test cases.
t = int(input())

# For each test case.
for _ in range(t):
    # Read the number of zucchinis.
    n = int(input())

    # Grab the heights of each zucchini in a list.
    heights = list(map(int, input().split(" ")))

    # Build our other list and initialize to False. We use two extra indices
    # to avoid having to do bounds checking.
    expect_next = [False] * (n + 2)

    # We keep track of the number of sequences we have found in `count`.
    count = 0

    # For each zucchini, if we have not seen the previous zucchini 
    # (with height - 1) we increase our count by 1. We then set the value for
    # our next zucchini (with height + 1) to True, indicating we expect to see
    # it further along in the sequence.
    for h in heights:
        if not expect_next[h]:
            count += 1
        expect_next[h + 1] = True

    # We take the ceiling of the log_2 of count...
    log = 0 
    while 1 << log < count:
        log += 1

    # And print out our answer.
    print(log)
```

## Conclusion

Wow, when I started writing this solution guide I did not expect to reach 10,000 words, let alone 20,000. I know that was a lot to read, but I hope if you stuck through it that you learned something that you may be able to apply in your schoolwork, a Code-a-Thon, a technical interview, or some other programming endeavor. These types of problems may not come up every day, but when one does, having an efficient solution can be all the difference in the world.

To those who participated in the Code-a-Thon, thank you for your participation. I love writing Code-a-Thon problems, but it would all be for naught if there were not bright students eager to solve them. Thanks again to the [University of South Carolina](https://sc.edu) and the [Department of Engineering and Computing](https://cse.sc.edu) for providing space and support for the Code-a-Thon. Thanks to USC-ACM for support in organization and advertising (and for being the computing club of choice for most of our participants)! Thanks to [Krumware](http://krum.io) for providing pizza in a solid attempt to placate the insatiable hunger of a room full of collegiate programmers. Lastly, I thank my wonderful Mom who proofread this entire post despite never before having read a single line of code.

If you have any questions or you have found any errors, please e-mail me at jadaytime (at) gmail.com. Unfortunately I can't send you a [hexadecimal dollar](https://en.wikipedia.org/wiki/Knuth_reward_check), but I will be very grateful nonetheless.

Thanks for reading!
