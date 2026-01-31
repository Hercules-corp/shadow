/**
 * Figma Tokens Extractor Helper
 * 
 * This script helps convert Figma design tokens into Tailwind CSS format.
 * 
 * Usage:
 * 1. Extract tokens from Figma (manually or via plugin)
 * 2. Create a figma-tokens.json file with your tokens
 * 3. Run: node scripts/extract-figma-tokens.js
 * 
 * This will generate:
 * - Updated globals.css with CSS variables
 * - Updated tailwind.config.ts with color scheme
 */

const fs = require('fs');
const path = require('path');

// Example tokens structure - replace with your actual Figma tokens
const figmaTokens = {
  colors: {
    primary: '#3B82F6', // Replace with your primary color
    secondary: '#8B5CF6',
    background: '#0A0A0A',
    foreground: '#FFFFFF',
    muted: '#3A3A3A',
    accent: '#10B981',
    destructive: '#EF4444',
    border: '#1F1F1F',
  },
  typography: {
    fontFamily: {
      sans: 'Inter, system-ui, sans-serif',
      mono: 'JetBrains Mono, monospace',
    },
    fontSizes: {
      xs: '12px',
      sm: '14px',
      base: '16px',
      lg: '18px',
      xl: '20px',
      '2xl': '24px',
      '3xl': '30px',
      '4xl': '36px',
    },
  },
  spacing: {
    scale: [4, 8, 12, 16, 20, 24, 32, 40, 48, 64],
  },
  borderRadius: {
    sm: '4px',
    md: '8px',
    lg: '12px',
    xl: '16px',
  },
};

// Convert hex to HSL for CSS variables
function hexToHsl(hex) {
  const r = parseInt(hex.slice(1, 3), 16) / 255;
  const g = parseInt(hex.slice(3, 5), 16) / 255;
  const b = parseInt(hex.slice(5, 7), 16) / 255;

  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  let h, s, l = (max + min) / 2;

  if (max === min) {
    h = s = 0; // achromatic
  } else {
    const d = max - min;
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    switch (max) {
      case r: h = ((g - b) / d + (g < b ? 6 : 0)) / 6; break;
      case g: h = ((b - r) / d + 2) / 6; break;
      case b: h = ((r - g) / d + 4) / 6; break;
    }
  }

  return `${Math.round(h * 360)} ${Math.round(s * 100)}% ${Math.round(l * 100)}%`;
}

// Generate CSS variables
function generateCSSVariables(tokens) {
  const colors = tokens.colors;
  const cssVars = [];

  // Color variables
  Object.entries(colors).forEach(([key, value]) => {
    if (typeof value === 'string' && value.startsWith('#')) {
      const hsl = hexToHsl(value);
      cssVars.push(`  --${key}: ${hsl};`);
      
      // Generate foreground if it's a primary/secondary color
      if (key === 'primary' || key === 'secondary') {
        const foreground = key === 'primary' ? colors.foreground : colors.foreground;
        const fgHsl = hexToHsl(foreground);
        cssVars.push(`  --${key}-foreground: ${fgHsl};`);
      }
    }
  });

  // Border radius
  cssVars.push(`  --radius: ${tokens.borderRadius?.md || '8px'};`);

  return cssVars.join('\n');
}

// Main execution
function main() {
  const tokensPath = path.join(__dirname, '..', 'figma-tokens.json');
  
  // Check if figma-tokens.json exists
  if (fs.existsSync(tokensPath)) {
    console.log('📦 Loading tokens from figma-tokens.json...');
    const customTokens = JSON.parse(fs.readFileSync(tokensPath, 'utf8'));
    Object.assign(figmaTokens, customTokens);
  } else {
    console.log('⚠️  No figma-tokens.json found. Using example tokens.');
    console.log('💡 Create figma-tokens.json with your Figma tokens to use this script.');
  }

  // Generate CSS variables
  const cssVars = generateCSSVariables(figmaTokens);
  
  console.log('\n📝 Generated CSS Variables:\n');
  console.log(cssVars);
  console.log('\n✅ Copy these to your globals.css file!');
  console.log('\n📋 Next steps:');
  console.log('1. Update frontend/app/globals.css with the CSS variables above');
  console.log('2. Update frontend/tailwind.config.ts if needed');
  console.log('3. Test your design!');
}

if (require.main === module) {
  main();
}

module.exports = { generateCSSVariables, hexToHsl };



