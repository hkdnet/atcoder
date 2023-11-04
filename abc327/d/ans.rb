def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N, M = getis
A = getis
B = getis
edges = Hash.new{|h,k|h[k] = {}}
A.zip(B).each do |a, b|
  edges[a-1][b-1] = true
  edges[b-1][a-1] = true
end

arr = Array.new(N) { -1 }
arr[0] = true
q = [0]
rest = Set.new
N.times {|e| rest << e }

f = true
while f && !rest.empty?
  # p [f, arr, rest]

  if q.empty?
    idx = rest.first
    arr[idx] = true
    q << idx
    next
  end

  cur = q.pop
  rest.delete(cur)
  edges[cur].keys.each do |i|
    if rest.include?(i)
      # new
      arr[i] = !arr[cur]
      q << i
    else
      # already visited
      if arr[i] == arr[cur]
        f = false
      end
    end
  end
end

if f
  puts "Yes"
else
  puts "No"
end
