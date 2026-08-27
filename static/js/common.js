export function clearErrors() {
    const errors = document.querySelectorAll(".error-message");

    errors.forEach((element) => {
        element.textContent = "";
    });
}


export function showError(field, message) {
    const errorElement =
        document.getElementById(`${field}-error`);

    if (errorElement) {
        errorElement.textContent = message;
    }
}