document.addEventListener('input', function (e) {
    if (e.target && e.target.id === 'password') {
        const password = e.target.value;
        const container = document.querySelector('.password-strength');
        const fill = document.getElementById('strength-fill');
        const text = document.getElementById('strength-text');

        if (password.length > 0) {
            container.style.display = 'block';
        } else {
            container.style.display = 'none';
            return;
        }

        let strength = 0;
        let messages = [];

        // Criteria
        if (password.length >= 8) strength++;
        if (/[A-Z]/.test(password)) strength++;
        if (/[0-9]/.test(password)) strength++;
        if (/[^A-Za-z0-9]/.test(password)) strength++;

        // Update UI
        fill.className = '';
        text.className = '';

        switch (strength) {
            case 0:
            case 1:
                fill.style.width = '33%';
                fill.classList.add('strength-weak');
                text.textContent = 'Weak (Add numbers, symbols, or uppercase)';
                text.classList.add('text-weak');
                break;
            case 2:
            case 3:
                fill.style.width = '66%';
                fill.classList.add('strength-medium');
                text.textContent = 'Medium';
                text.classList.add('text-medium');
                break;
            case 4:
                fill.style.width = '100%';
                fill.classList.add('strength-strong');
                text.textContent = 'Strong';
                text.classList.add('text-strong');
                break;
        }
    }
});
