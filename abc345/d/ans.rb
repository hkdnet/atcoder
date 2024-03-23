def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, H, W = getis
tiles = N.times.map do
  getis
end

def solve(tiles, sqs)
  (2**N).times do |s|
    ts = tiles.each.with_index.map do |(a, b), idx|
      if (s >> idx) & 1 == 1
        [a, b]
      else
        [b, a]
      end
    end
    sqs = [[N, N]]
    ts.each do |a, b|

    end

  end
end
# (2**N).times do |s|
#   ts = tiles.each.with_index.map do |(a, b), idx|
#     if (s >> idx) & 1 == 1
#       [a, b]
#     else
#       [b, a]
#     end
#   end
#   sqs = [[N, N]]
#   ts.each do |a, b|

#   end
# end
if solve(tiles, [[N, N]])
  puts "Yes"
else
  puts "No"
end
