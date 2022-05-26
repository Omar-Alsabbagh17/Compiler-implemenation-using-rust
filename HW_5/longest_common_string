
lcp :: Eq str => [[str]] -> [str]
{-
Accepts list of string as paramater
and returns the longest common prefix in that list
-}
lcp = foldl1 checker 

checker :: (Eq char) => [char] -> [char] -> [char]
checker _ [] = []
checker [] _ = []

checker (x : x_s) (y : y_s)
    {-
    if two character are same then
    then add to lcp
    otherwise we reached end
    -}
    | x == y = x : checker x_s y_s
    | otherwise = []
    
------------ main -----------------
main:: IO()
main = do
    print (lcp ["abcd", "abcd", "abcdbbb", "abcdbbbb"])
    print (lcp["apple", "app", "aple", "appl"])
    print (lcp["a", "b", "c", "d"])
    print (lcp["orange_melon", "orange_apple", "orange", "orange_cherry"])
    print (lcp [""])
