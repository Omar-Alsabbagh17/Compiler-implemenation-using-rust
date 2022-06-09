%  Represent all the vertices in the above graph using the vertex relation
vertex(a).
vertex(b).
vertex(c).
vertex(d).


% Represent all the edges in the above graph using the edge relation
edge(a,b).
edge(a,c).
edge(c,b).
edge(c,d).
edge(d,c).



% 3. output of edge(b,c) will be "no" meaning there is no edge from b to c.


% 4. paths

path(X,Y):- edge(X,Y) ; ( edge(X,Z) , path(Z,Y)).
