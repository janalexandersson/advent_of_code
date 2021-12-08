input <- scan("input.txt")
sum(diff(input, lag = 1) > 0)
#1475