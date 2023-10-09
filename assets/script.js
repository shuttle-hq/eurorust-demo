const greetingMessage = document.getElementById('greetingMessage');

// Fetch the greetings total and set it in the HTML.
async function fetchGreetings() {
  try {
    const response = await fetch(`/greeting`);
    const greetings = await response.json();

    greetingMessage.innerHTML = greetings + ' people said hello from EuroRust!';
  } catch (error) {
    console.error(`Error fetching greetings`, error);
    return null;
  }
}

async function addGreeting() {
  try {
    const response = await fetch('/greeting', {
      method: 'PUT',
    });
    if (response.ok) {
      let greetings = await response.json();
      greetingMessage.innerHTML =
        greetings + ' people said hello from EuroRust!';
    } else {
      console.error('Failed to add greeting.');
    }
  } catch (error) {
    console.error('Error adding greeting:', error);
  }
}

fetchGreetings();
