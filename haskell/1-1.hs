main = do
  interact solution

solution :: String -> String
solution s = show $ foldl move 0 s

move :: Int -> Char -> Int
move num '(' = num + 1
move num ')' = num - 1