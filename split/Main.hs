charIndices :: String -> Char -> [Int]
charIndices xs c = go 0 xs c []
  where
    go _ [] _ acc  = foldl (\acc x -> x : acc) [] acc
    go idx (x:xs) c acc
      | x == c = go (idx + 1) xs c (idx : acc)
      | otherwise = go (idx + 1) xs c acc

split :: String -> Char -> [String]
split xs c = 
  let indices = charIndices xs c
  in let (idx, res, end) = (foldl (\ (start, res, xs) end -> (end + 1, (take (end - start) xs) : res, drop (end - start + 1) xs)) (0, [], xs)) indices
  in reverse (end : res)
main = do
    let str = "4f:5c:56:ac:fe:5d" 
    print $ str
    print $ split str ':'
