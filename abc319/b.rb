N = gets.to_i

arr = 1.upto(9).to_a
arr = arr.select {|e| N % e == 0 }

0.upto(N) do |i|
  f = false
  arr.each do |j|
    if i % (N/j) == 0
      print j
      f = true
      break
    end
  end
  if !f
    print '-'
  end
end

puts
