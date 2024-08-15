/**
 * This function handles the toggling of selections of all upstream search engines
 * options in the settings page under the tab engines.
 */
function toggleAllSelection() {
  document
    .querySelectorAll('.engine')
    .forEach(
      (engine_checkbox) =>
      (engine_checkbox.checked =
        document.querySelector('.select_all').checked),
    )
}

/**
 * This function adds the functionality to sidebar buttons to only show settings
 * related to that tab.
 * @param {HTMLElement} current_tab - The current tab that was clicked.
 */
function setActiveTab(current_tab) {
  // Remove the active class from all tabs and buttons
  document
    .querySelectorAll('.tab')
    .forEach((tab) => tab.classList.remove('active'))
  document
    .querySelectorAll('.btn')
    .forEach((tab) => tab.classList.remove('active'))

  // Add the active class to the current tab and its corresponding settings
  current_tab.classList.add('active')
  document
    .querySelector(`.${current_tab.innerText.toLowerCase().replace(' ', '_')}`)
    .classList.add('active')
}

/**
 * This function adds the functionality to save all the user selected preferences
 * to be saved in a cookie on the users machine.
 */
function setClientSettings() {
  // Create an object to store the user's preferences
  let cookie_dictionary = new Object()

  document.querySelectorAll('.engine').forEach((engine_checkbox) => {
    cookie_dictionary[engine_checkbox.parentNode.parentNode.innerText.trim()] = engine_checkbox.checked
  })

  // Set the expiration date for the cookie to 1 year from the current date
  let expiration_date = new Date()
  expiration_date.setFullYear(expiration_date.getFullYear() + 1)

  // Save the cookie to the user's machine
  document.cookie = `appCookie=${JSON.stringify(
    cookie_dictionary,
  )}; expires=${expiration_date.toUTCString()}`

  // Display a success message to the user
  document.querySelector('.message').innerText =
    '✅ The settings have been saved sucessfully!!'

  // Clear the success message after 10 seconds
  setTimeout(() => {
    document.querySelector('.message').innerText = ''
  }, 10000)
}

