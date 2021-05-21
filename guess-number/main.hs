import System.Random
game :: Int -> Int -> IO Int
game correct counter = do
    guess <- getLine
    case correct `compare` (read guess :: Int) of 
        LT -> do
             putStrLn "Less."
             game correct (counter + 1)
        GT -> do
             putStrLn "More !"
             game correct (counter + 1)
        EQ -> return counter
main = do
    num <- randomRIO (1, 100) :: IO Int
    counter <- game num 1
    putStrLn $ "You found the number in " ++ show counter ++ " tries."
