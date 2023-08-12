main = do
  interact solution

solution = show . foldl move 0
  where
    move acc '(' = acc + 1
    move acc ')' = acc - 1