
console.log("REGISTER JS LOADED");

const form = document.getElementById("register-form");

form.addEventListener("submit", async (event) => {
    event.preventDefault();

    console.log("REGISTER FORM SUBMITTED");

    // Clear previous errors
    document.querySelectorAll(".error-message").forEach((element) => {
        element.textContent = "";
    });

    const formData = new FormData(form);
    const body = new URLSearchParams(formData);

    console.log("Sending registration request...");

    try {
        const response = await fetch("/api/auth/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded"
            },
            body: body
        });

        console.log("Response:", response.status);

        // Registration failed
        if (!response.ok) {
            const result = await response.json();

            console.log("Server response:", result);

            const errorElement = document.getElementById(
                `${result.field}-error`
            );

            console.log("Error element:", errorElement);

            if (errorElement) {
                errorElement.textContent = result.message;
            }

            return;
        }

        // Registration successful
        console.log("Registration successful");

        window.location.href = "/auth/home";

    } catch (error) {
        console.error("Request failed:", error);

        const generalError = document.getElementById("general-error");

        if (generalError) {
            generalError.textContent =
                "Unable to connect to the server. Please try again later";
        }
    }
});