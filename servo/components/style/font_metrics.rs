/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Access to font metrics from the style system.

#![deny(missing_docs)]

use crate::context::SharedStyleContext;
use crate::values::computed::Length;
use crate::Atom;

/// Represents the font metrics that style needs from a font to compute the
/// value of certain CSS units like `ex`.
#[derive(Clone, Debug, PartialEq)]
pub struct FontMetrics {
    /// The x-height of the font.
    pub x_height: Option<Length>,
    /// The zero advance. This is usually writing mode dependent
    pub zero_advance_measure: Option<Length>,
    /// The cap-height of the font.
    pub cap_height: Option<Length>,
    /// The ideographic-width of the font.
    pub ic_width: Option<Length>,
    /// The ascent of the font (a value is always available for this).
    pub ascent: Length,
}

impl Default for FontMetrics {
    fn default() -> Self {
        FontMetrics {
            x_height: None,
            zero_advance_measure: None,
            cap_height: None,
            ic_width: None,
            ascent: Length::new(0.0),
        }
    }
}

/// Type of font metrics to retrieve.
#[derive(Clone, Debug, PartialEq)]
pub enum FontMetricsOrientation {
    /// Get metrics for horizontal or vertical according to the Context's
    /// writing mode, using horizontal metrics for vertical/mixed
    MatchContextPreferHorizontal,
    /// Get metrics for horizontal or vertical according to the Context's
    /// writing mode, using vertical metrics for vertical/mixed
    MatchContextPreferVertical,
    /// Force getting horizontal metrics.
    Horizontal,
}

/// A trait used to represent something capable of providing us font metrics.
pub trait FontMetricsProvider {
    /// Obtain the metrics for given font family.
    fn query(
        &self,
        _context: &crate::values::computed::Context,
        _base_size: crate::values::specified::length::FontBaseSize,
        _orientation: FontMetricsOrientation,
    ) -> FontMetrics {
        Default::default()
    }

    /// Get default size of a given language and generic family.
    fn get_size(
        &self,
        font_name: &Atom,
        font_family: crate::values::computed::font::GenericFontFamily,
    ) -> Length;

    /// Construct from a shared style context
    fn create_from(context: &SharedStyleContext) -> Self
    where
        Self: Sized;
}

// TODO: Servo's font metrics provider will probably not live in this crate, so this will
// have to be replaced with something else (perhaps a trait method on TElement)
// when we get there
#[derive(Debug)]
#[cfg(feature = "servo")]
/// Dummy metrics provider for Servo. Knows nothing about fonts and does not provide
/// any metrics.
pub struct ServoMetricsProvider;

#[cfg(feature = "servo")]
impl FontMetricsProvider for ServoMetricsProvider {
    fn create_from(_: &SharedStyleContext) -> Self {
        ServoMetricsProvider
    }

    fn get_size(&self, _: &Atom, _: crate::values::computed::font::GenericFontFamily) -> Length {
        unreachable!("Dummy provider should never be used to compute font size")
    }
}

// Servo's font metrics provider will probably not live in this crate, so this will
// have to be replaced with something else (perhaps a trait method on TElement)
// when we get there

#[cfg(feature = "gecko")]
/// Construct a font metrics provider for the current product
pub fn get_metrics_provider_for_product() -> crate::gecko::wrapper::GeckoFontMetricsProvider {
    crate::gecko::wrapper::GeckoFontMetricsProvider::new()
}

#[cfg(feature = "servo")]
/// Construct a font metrics provider for the current product
pub fn get_metrics_provider_for_product() -> ServoMetricsProvider {
    ServoMetricsProvider
}
