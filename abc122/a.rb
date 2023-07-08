b = gets.chomp
ret = {
  ?A => ?T,
  ?T => ?A,
  ?C => ?G,
  ?G => ?C,
}.fetch(b)
puts ret
