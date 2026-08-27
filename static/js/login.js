console.log("LOGIN JS LOADED");

const form = document.getElementById("login-form");

form.addEventListener("submit", async (event) => {
    event.preventDefault();

    console.log("LOGIN FORM SUBMITTED");

    // Clear previous errors
    document.querySelectorAll(".error-message").forEach((element) => {
        element.textContent = "";
    });

    const formData = new FormData(form);
    const body = new URLSearchParams(formData);

    console.log("Sending login request...");

    try {
        const response = await fetch("/api/auth/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded"
            },
            body: body
        });

        console.log("Response:", response.status);

        const result = await response.json();

        console.log("Server response:", result);

        // Login failed
        if (!response.ok) {
            const errorElement = document.getElementById(
                `${result.field}-error`
            );

            console.log("Error element:", errorElement);

            if (errorElement) {
                errorElement.textContent = result.message;
            }

            return;
        }

        // Login successful
        console.log("Login successful");

        window.location.href = "/auth/home";

    } catch (error) {
        console.error("Request failed:", error);

        const generalError = document.getElementById("general-error");

        if (generalError) {
            generalError.textContent =
                "Unable to connect to the server. Please try again later.";
        }
    }
});
