N = gets.to_i
as = gets.chomp.split(" ").map(&:to_i)

ans = 0
1.upto(N) do |i|
  (i + 1).upto(N) do |j|
    (j + 1).upto(N) do |k|
      if as[i] == as[k] && as[i] != as[j]
        ans += 1
      end
    end
  end
end

puts ans
