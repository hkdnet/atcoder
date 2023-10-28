def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
products = N.times.map do
  t, d = getis
  [t, t + d]
end
products.sort! # 6WA
# products.sort_by! {|arr| [arr[1], arr[0]]} # 24WA

cur = 1
ans = 0
products.each do |t, d|
  # p [t, d, cur]
  if cur <= d
    # puts "ikeru"
    if cur < t
      cur = t
    end
    ans += 1
    cur += 1
  else
    # puts "ikenai"
  end
end

puts(ans)
