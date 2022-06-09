rev([], _, []).
rev([[X|Y]|Z], X, [Y|Remaining]) :-  rev(Z, X, Remaining).

common_prefix(Y,L) :- rev(L, X, L1),
	Y = [X|Z],
	common_prefix(Z,L1);
	Y = [].
