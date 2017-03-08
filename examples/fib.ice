func fib (a: int) -> int
    if a < 3
        return a
    end

    return (fib a - 1) + (fib a - 2)
end