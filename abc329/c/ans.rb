def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = gets.chomp
cs = S.chars
l = Hash.new{|h, k| h[k] = 0}
cur = nil
cnt = 0
cs.each do |c|
  if cur == c
    cnt += 1
  else
    l[cur] = cnt if cnt > l[cur]
    cur = c
    cnt = 1
  end
end
l[cur] = cnt if cnt > l[cur]
l.delete(nil)
puts(l.values.sum)
