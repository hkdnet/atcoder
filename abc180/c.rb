n = gets.to_i

m = Math.sqrt(n).ceil
ans = {}

1.upto(m) do |e|
  if n % e == 0
    ans[e] = true
    ans[n/e] = true
  end
end

ans.keys.sort.each { |e| puts e }
