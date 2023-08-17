import Crypto.Hash
import Data.ByteString (ByteString)
import qualified Data.ByteString.Char8 as C8
import qualified Data.ByteString.Lazy as LB
import Data.List

main :: IO ()
main = interact $ show . parse 0

md5 :: String -> Int -> String
md5 s x = show . hashmd5 . C8.pack $ s ++ show x

hashmd5 :: ByteString -> Digest MD5
hashmd5 = hashlazy . LB.fromStrict

parse :: Int -> String -> Int
parse x s
  | "00000" `isPrefixOf` md5 s x = x
  | otherwise = parse (x + 1) s