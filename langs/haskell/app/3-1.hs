import Data.Set (Set, empty, insert)

main :: IO ()
main = interact $ solution (Point 0 0) empty

data Point = Point Int Int deriving (Show, Eq, Ord)

solution :: Point -> Set Point -> String -> String
solution _ s [] = show . length $ s
solution p s (x : xs) = solution (next x p) (insert p s) xs

next :: Char -> Point -> Point
next '^' (Point x y) = Point (x + 1) y
next 'v' (Point x y) = Point (x - 1) y
next '>' (Point x y) = Point x (y + 1)
next '<' (Point x y) = Point x (y - 1)