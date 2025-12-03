module Main where
import Data.Char (digitToInt)

main :: IO ()
main = do
    contents <- readFile "input.txt"
    let rows = lines contents
    let updated = map part1 rows
    print (sum updated)



part1 :: String -> Int
part1 str = maximum combinations
  where 
    digits = map digitToInt str
    combinations = [d1 * 10 + d2 | (i, d1) <- zip [0..] digits
                                    , (j, d2) <- zip [0..] digits
                                    , i < j]