use actix_web::{web, App, HttpServer, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Portfolio - Muhammad Adeel</title>
                <style>
                    body {
                        font-family: Arial, sans-serif;
                        line-height: 1.6;
                        margin: 0;
                        padding: 0;
                        background-color: #f4f4f4;
                        color: #333;
                    }
                    header {
                        background: #333;
                        color: #fff;
                        padding: 20px;
                        position: fixed;
                        width: 100%;
                        top: 0;
                        left: 0;
                        box-shadow: 0 2px 5px rgba(0,0,0,0.3);
                        z-index: 1000;
                    }
                    header h1 {
                        margin: 0;
                        font-size: 2rem;
                        text-align: center;
                        animation: fadeIn 2s ease-out;
                    }
                    nav ul {
                        display: flex;
                        justify-content: center;
                        list-style: none;
                        margin: 0;
                        padding: 0;
                    }
                    nav ul li {
                        margin: 0 15px;
                    }
                    nav ul li a {
                        color: #fff;
                        text-decoration: none;
                        font-weight: bold;
                        transition: color 0.3s;
                    }
                    nav ul li a:hover {
                        color: #ffa500;
                    }
                    .container {
                        width: 80%;
                        margin: 0 auto;
                        padding: 60px 0 20px; /* Added top padding for fixed header */
                    }
                    .main-content {
                        background: #fff;
                        padding: 20px;
                        margin: 20px 0;
                        border-radius: 8px;
                        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                        animation: fadeInUp 1s ease-out;
                    }
                    .contact-info {
                        margin-top: 20px;
                    }
                    footer {
                        text-align: center;
                        padding: 10px;
                        background: #333;
                        color: #fff;
                        position: fixed;
                        bottom: 0;
                        width: 100%;
                        animation: fadeIn 2s ease-out;
                    }
                    /* Animations */
                    @keyframes fadeIn {
                        from { opacity: 0; }
                        to { opacity: 1; }
                    }
                    @keyframes fadeInUp {
                        from { opacity: 0; transform: translateY(20px); }
                        to { opacity: 1; transform: translateY(0); }
                    }
                    @keyframes slideIn {
                        from { transform: translateX(-20px); opacity: 0; }
                        to { transform: translateX(0); opacity: 1; }
                    }
                    /* Responsive */
                    @media (max-width: 768px) {
                        .container {
                            width: 95%;
                        }
                        header h1 {
                            font-size: 1.5rem;
                        }
                        nav ul {
                            flex-direction: column;
                        }
                        nav ul li {
                            margin: 10px 0;
                        }
                    }
                </style>
            </head>
            <body>
                <header>
                    <div class="container">
                        <h1>Portfolio - Muhammad Adeel</h1>
                        <nav>
                            <ul>
                                <li><a href="about">About</a></li>
                                <li><a href="skills">Skills</a></li>
                                <li><a href="contact">Contact</a></li>
                                <li><a href="projects">Projects</a></li>
                            </ul>
                        </nav>
                    </div>
                </header>

                <div class="container main-content" id="about">
                    <h2>About Me</h2>
                    <p>I am a Java front-end and back-end developer, involved in open-source contributions, problem-solving on LeetCode, web development, and Flutter development. I have participated in Hacktoberfest, Hackersquad, and various projects. I am passionate about expanding my open-source contributions through GSOC and other opportunities. I believe in learning and growing every day and value hard work and consistency.</p>
                </div>

                <div class="container main-content" id="skills">
                    <h2>Skills</h2>
                    <ul>
                        <li>Java (Spring Boot, REST API, Java GUI, Swing, JavaFX)</li>
                        <li>C++</li>
                        <li>Python3 (NumPy)</li>
                        <li>HTML, CSS, JavaScript</li>
                        <li>SQL</li>
                        <li>Flutter Development (Dart)</li>
                    </ul>
                </div>

                <div class="container main-content" id="contact">
                    <h2>Contact Information</h2>
                    <form id="contactForm">
                        <label for="name">Name:</label>
                        <input type="text" id="name" name="name" required><br><br>
                        <label for="email">Email:</label>
                        <input type="email" id="email" name="email" required><br><br>
                        <label for="message">Message:</label><br>
                        <textarea id="message" name="message" rows="4" required></textarea><br><br>
                        <input type="submit" value="Send">
                    </form>
                    <div id="formResponse"></div>
                </div>

                <div class="container main-content" id="projects">
                    <h2>Projects</h2>
                    <ul>
                        <li>University Projects: Scientific Calculator, Stopwatch, TicTacToe</li>
                        <li>CODSOFT Projects: Random Number Generator, Students Marks Calculator, ATM Banking System</li>
                    </ul>
                </div>

                <footer>
                    <p>&copy; 2024 Muhammad Adeel. All Rights Reserved.</p>
                </footer>

                <script>
                    document.getElementById('contactForm').addEventListener('submit', function(event) {
                        event.preventDefault();
                        document.getElementById('formResponse').innerText = 'Thank you for your message! I will get back to you soon.';
                        document.getElementById('contactForm').reset();
                    });
                </script>
            </body>
            </html>
        "#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}