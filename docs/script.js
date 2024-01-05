let cart = {}

function update(key) {
	document.querySelector(`#${key} #count`).innerHTML = `Count: ${cart[key]}`
	}

function addToCart(key) {
	if (cart[key] === undefined) cart[key] = 1;
	else cart[key] += 1;

	update(key)
}

function removeFromCart(key) {
	if (cart[key] !== undefined && cart[key] > 0) cart[key] -= 1;

	update(key)
}

const cards = document.querySelectorAll('.card')

for(const card of cards) {
	const id = card.getAttribute('id')
	const add = card.querySelector('button#add')
	const remove = card.querySelector('button#remove')

	add.addEventListener('click', () => addToCart(id));
	remove.addEventListener('click', () => removeFromCart(id))
}
