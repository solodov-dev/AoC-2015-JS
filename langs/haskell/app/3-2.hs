import Data.Set (Set, empty, insert, union)

main :: IO ()
main = interact $ solution (Point 0 0) (Point 0 0) empty empty

data Point = Point Int Int deriving (Show, Eq, Ord)

solution :: Point -> Point -> Set Point -> Set Point -> String -> String
solution p1 p2 s r [] = show . length $ insert p1 s `union` insert p2 r
solution p1 p2 s r (x : y : xs) = solution (next x p1) (next y p2) (insert p1 s) (insert p2 r) xs

next :: Char -> Point -> Point
next '^' (Point x y) = Point (x + 1) y
next 'v' (Point x y) = Point (x - 1) y
next '>' (Point x y) = Point x (y + 1)
next '<' (Point x y) = Point x (y - 1)