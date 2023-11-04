def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end

N = geti
S = gets.chomp.chars
Q = geti
l = {}
u = {}
last = nil
Q.times do
  t, x, c = gets.chomp.split(" ")
  t = t.to_i
  case t
  when 1
    x = x.to_i - 1
    S[x] = c
    if c < 'a'
      u[x] = true
      l[x] = false
    else
      l[x] = true
      u[x] = false
    end
  when 2
    u = {}
    last = :l
  when 3
    l = {}
    last = :u
  end
end

S.each.with_index do |c, i|
  if l[i]
    print(c.downcase)
  elsif u[i]
    print(c.upcase)
  else
    case last
    when :l
      print(c.downcase)
    when :u
      print(c.upcase)
    else
      print(c)
    end
  end
end

puts ""
