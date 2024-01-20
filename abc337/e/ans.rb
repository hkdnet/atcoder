$stdout.sync = true
def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti

h = Hash.new { |h, k| h[k] = [] }
N.times do |i|
  s = i.to_s(2)
  s.chars.reverse_each.with_index(1) do |c, idx|
    if c == '1'
      h[idx] << i + 1
    end
  end
end
M = h.keys.max

$stdout.puts M

1.upto(M) do |k|
  arr = h[k]
  puts "#{arr.size} #{arr.join(" ")}"
end

s = gets.chomp
ans = s.reverse.to_i(2)

$stdout.puts ans + 1
