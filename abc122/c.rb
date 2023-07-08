_, Q = gets.chomp.split(' ').map(&:to_i)
S = gets.chomp

acc = []
acc[0] = 0
1.upto(S.size + 1).each do |idx|
  acc[idx] = acc[idx - 1]
  if S[idx] == ?C && S[idx - 1] == ?A
    acc[idx] += 1
  end
end


Q.times do
  l, r = gets.chomp.split(' ').map(&:to_i).map {|e| e - 1 }
  ans = acc[r]
  if l != 0
    ans -= acc[l-1]
    if S[l] == "C" && S[l-1] == "A"
      ans -= 1
    end
  end
  puts ans
end
