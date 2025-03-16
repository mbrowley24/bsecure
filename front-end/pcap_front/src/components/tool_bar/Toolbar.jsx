import { useState } from "react";
import { Menu, X, Search, Bell, User } from "lucide-react";
import "../../css/toolbar.css";

const  Toolbar = () => {
    const [menuOpen, setMenuOpen] = useState(false);

    return (
        <header className="toolbar">
            {/* Left Side: Logo & Menu Button */}
            <div className="toolbar-left">
                <button className="menu-button" onClick={() => setMenuOpen(!menuOpen)}>
                    {menuOpen ? <X /> : <Menu />}
                </button>
                <div className="logo">MyApp</div>
            </div>

            {/* Navigation Links (Hidden on Mobile, Visible on Desktop) */}
            <nav className={`nav-links ${menuOpen ? "open" : ""}`}>
                <a href="/" onClick={() => setMenuOpen(false)}>Home</a>
                <a href="/about" onClick={() => setMenuOpen(false)}>About</a>
                <a href="/services" onClick={() => setMenuOpen(false)}>Services</a>
                <a href="/contact" onClick={() => setMenuOpen(false)}>Contact</a>
            </nav>

            {/* Right-side Icons */}
            <div className="toolbar-right">
                <div className="search-box">
                    <Search className="search-icon" />
                    <input type="text" placeholder="Search..." />
                </div>
                <button className="icon-button">
                    <Bell />
                    <span className="badge"></span>
                </button>
                <button className="icon-button">
                    <User />
                </button>
            </div>
        </header>
    );
}

export default Toolbar;
