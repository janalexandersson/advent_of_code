# part 1
input <- scan("input.txt")
number_of_increases <- function(x) sum(diff(x, lag = 1) > 0, na.rm = TRUE)
number_of_increases(input)
#1475

# part 2
rolling_sum <- function(x) sapply(1:length(input), FUN = function(i) sum(input[i:(i+2)]))
number_of_increases(rolling_sum(input))