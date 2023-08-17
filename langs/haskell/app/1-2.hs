main :: IO ()
main = interact $ show . length . takeWhile (>= 0) . scanl move 0
  where
    move acc '(' = acc + 1
    move acc ')' = acc - 1