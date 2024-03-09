def geti
  gets.chomp.to_i
end

def getis
  gets.chomp.split(" ").map(&:to_i)
end


N = geti
A = getis
h = {}
# [val, prev, next]
head = { val: A[0], prev: nil, next: nil }
h[A[0]] = head
tmp = head
A.drop(1).each do |a|
  node = { val: a, prev: tmp, next: nil }
  h[a] = node
  tmp[:next] = node
  tmp = node
end

Q = geti

def l_to_a(head)
  a = []
  while head
    a << head[:val]
    head = head[:next]
  end
  a
end

Q.times do
  arr = getis
  if arr[0] == 1
    x = arr[1]
    y = arr[2]
    node = h.fetch(x)
    next_node = node[:next]
    # node - new_node - next_node
    new_node = { val: y, prev: node, next: next_node }

    node[:next] = new_node
    next_node[:prev] = new_node if next_node

    h[y] = new_node
  else
    x = arr[1]
    node = h.fetch(x)
    prev = node[:prev]
    next_node = node[:next]
    if prev
      prev[:next] = next_node
      next_node[:prev] = prev if next_node
    else
      # deleting the head
      head = node[:next]
      head[:prev] = nil
    end
    h.delete(x)
  end

  # p h
  # p l_to_a(head)
end


a = l_to_a(head)
puts a.join(" ")
