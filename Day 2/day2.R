input <- as.data.frame(read.table("input.txt", sep = " "))
horiz_pos = 0
depth = 0
input[3,2]
for (i in nrow(input)){
  print(i)
  print(input[i,1])
  switch(
    input[i,1],
    "forward"={horiz_pos = horiz_pos + input[i,2]},
    "down"={depth = depth - input[i,2]},
    "up"={depth = depth + input[i,2]}
  )
}
print(horiz_pos*depth)

typeof(input)
