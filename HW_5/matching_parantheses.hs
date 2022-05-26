matching :: String -> Bool
{-
    This function accepts a string and checks whether it has matching
    parantheses or not. That's it, it checks whether each opening paranthese
    it's own closing parantheses in correct order.
    The function ignores letters, numbers, and any other character
    and only checks the following parantheses:
                {}, (), []
                
    example:
    ([]) : true
    ([) : false, since "[" is missing it's closing parntheses "]"
    ()ab{}123: true
-}
matching x_s = var x_s ""
    where var "" stack = null stack
          var (']':x_s)  ('[':y_s)  =  var  x_s y_s
          var (')':x_s) ('(':y_s) = var  x_s y_s
          var ('}':x_s)  ('{':y_s)  = var  x_s y_s
          var (x:x_s) stack
                   | elem x "])}" = False
                | elem x  "[({" = var x_s (x:stack)
              | otherwise = var x_s stack

--------- main -------------
main :: IO()
main = do
    --- test cases
    let res = matching("[]")
    print res
    let res = matching("aa")
    print res
    let res = matching("[{}()abc]")
    print res
    let res = matching("[((((")
    print res
    let res = matching("[({})")
    print res
    let res = matching("({})")
    print res
