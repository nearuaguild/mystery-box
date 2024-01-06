/**
 * Generates a URL to a widget.
 *
 * @param {Object} options - Configuration options for constructing the URL.
 * @param {string} options.widgetSrc - The source path of the widget (required).
 * @param {Object} [options.params] - An object containing key-value pairs representing query parameters to be appended to the URL (optional).
 * @returns {string} - The constructed URL.
 */
function href({ widgetSrc, params }) {
  // Check if query parameters are provided and filter out null values
  if (params) {
    params = (Object.entries(params) || [])
      .filter(([_key, nullable]) => (nullable ?? null) !== null)
      .map(([key, value]) => {
        // Omit the parameter if the value is null or the array is empty
        if (value === null || (Array.isArray(value) && value.length === 0)) {
          return null;
        }

        // Convert array values to a comma-separated string with no spaces
        if (Array.isArray(value)) {
          return `${key}=${value.join(',')}`;
        } else {
          return `${key}=${value}`;
        }
      })
      .join('&');
  }

  return `/${widgetSrc}${params && `?${params}`}`;
}

return { href };
