N = gets.to_i
as = gets.chomp.split(" ").map(&:to_i)

ans = 0
0.upto(N-1) do |i|
  (i + 1).upto(N-1) do |j|
    (j + 1).upto(N-1) do |k|
      if as[i] == as[k] && as[i] != as[j]
        ans += 1
      end
    end
  end
end

puts ans
