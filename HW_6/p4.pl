my_reverse([],[]). % in case empty list
my_reverse([First|Remaining],Reversed) :-
 my_reverse(Remaining, RevRemaining),chain(RevRemaining, [First],Reversed).
chain([],Z,Z).
chain([X1|Y1],Y2,[X1|Y3]) :- chain(Y1,Y2,Y3).