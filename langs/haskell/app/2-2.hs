main :: IO ()
main = interact $ show . sum . map (calc . split) . lines

split :: String -> [String]
split str = words [if c == 'x' then ' ' else c | c <- str]

calc :: (Ord a, Read a, Num a) => [String] -> a
calc (length : width : height : _) = minimum perimeters + vol
  where
    [l, w, h] = map read [length, width, height]
    perimeters = map (* 2) [l + w, w + h, h + l]
    vol = l * w * h
