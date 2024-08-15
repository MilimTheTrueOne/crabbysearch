/**
 * This functions gets the saved cookies if it is present on the user's machine If it
 * is available then it is parsed and converted to an object which is then used to
 * retrieve the preferences that the user had selected previously and is then loaded
 * and used for displaying the user provided settings by setting them as the selected
 * options in the settings page.
 *
 * @function
 * @param {string} cookie - It takes the client settings cookie as a string.
 * @returns {void}
 */
function setClientSettingsOnPage(cookie) {
  // Loop through all select tags and add their values to the cookie dictionary
  let engines = document.querySelectorAll('.engine')

  document.querySelector('.select_all').checked = true

  engines.forEach((engine) => {
    engine.checked = cookie[engine.parentNode.parentNode.innerText.trim()];
    if (!engine.checked) {
      document.querySelector('.select_all').checked = false
    }
  })

}

/**
 * This function is executed when any page on the website finishes loading and
 * this function retrieves the cookies if it is present on the user's machine.
 * If it is available then the saved cookies is display in the cookies tab
 * otherwise an appropriate message is displayed if it is not available.
 *
 * @function
 * @listens DOMContentLoaded
 * @returns {void}
 */
document.addEventListener(
  'DOMContentLoaded',
  () => {
    try {
      // Decode the cookie value
      let cookie = decodeURIComponent(document.cookie)
      // Set the value of the input field to the decoded cookie value if it is not empty
      // Otherwise, display a message indicating that no cookies have been saved on the user's system
      if (cookie.length) {
        document.querySelector('.cookies input').value = cookie
        // This function displays the user provided settings on the settings page.
        setClientSettingsOnPage(JSON.parse(cookie.replace("appCookie=", "")))
      } else {
        document.querySelector('.cookies input').value =
          'No cookies have been saved on your system'
      }
    } catch (error) {
      // If there is an error decoding the cookie, log the error to the console
      // and display an error message in the input field
      console.error('Error decoding cookie:', error)
      document.querySelector('.cookies input').value = 'Error decoding cookie'
    }
  },
  false,
)
