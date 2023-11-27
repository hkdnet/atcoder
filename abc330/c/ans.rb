def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

D = geti

arr = [0]
1.upto(10**13) do |a|
  arr[a] = a**2
  break if arr[a] > D
end

def bin_search(l, r)
  loop do
    idx = (l + r)/2
    return idx if idx == l || idx == r
    f = yield idx
    if f
      l = idx
    else
      r = idx
    end
  end
end


ans = nil
0.upto(arr.size-1) do |l|
  r = bin_search(l, arr.size-1) do |r|
    arr[l] + arr[r] <= D
  end
  tmp = (arr[l] + arr[r] - D).abs
  ans = tmp if ans.nil? || tmp < ans
  if r + 1 < arr.size
    tmp = (arr[l] + arr[r+1] - D).abs
    ans = tmp if tmp < ans
  end
end

puts ans
