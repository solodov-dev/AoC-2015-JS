main = do
  interact solution

solution :: String -> String
solution = show . sum . map (calc . words . replace) . lines

replace :: String -> String
replace str = [if c == 'x' then ' ' else c | c <- str]

calc :: (Ord a, Read a, Num a) => [String] -> a
calc (length : width : height : _) = minimum perimeters + vol
  where
    [l, w, h] = map read [length, width, height]
    perimeters = map (* 2) [l + w, w + h, h + l]
    vol = l * w * h
