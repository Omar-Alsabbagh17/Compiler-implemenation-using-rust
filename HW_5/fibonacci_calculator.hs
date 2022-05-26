fib::Int->Int  -- this funtion calculates the fibonacci number
fib 0 = 0    -- base case
fib 1 = 1    -- base case
fib n = fib (n-1) + fib (n-2)  -- recursive cases
fibSeq n = map fib [0..n]  -- Create a list containing till the nth element of  fibonacci numbers 

-----------  main  -----------------------
main :: IO ()
main = do
    let res = fibSeq(12)
    print res
