import Data.List (elemIndex)
import Data.Maybe (fromMaybe)

main = do
  interact solution

solution :: String -> String
solution s = show $ fromMaybe (-1) . elemIndex (-1) $ scanl move 0 s

move :: Int -> Char -> Int
move num '(' = num + 1
move num ')' = num - 1
