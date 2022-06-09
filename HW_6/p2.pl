is_palindrome(Rem_X) :- is_palindrome(Rem_X,Rem_X,[]).
is_palindrome([],Rem_X,Rem_X).
is_palindrome([Y|Rem_Y],Rem_X,Rem_Z):- is_palindrome(Rem_Y,Rem_X,[Y|Rem_Z]).





