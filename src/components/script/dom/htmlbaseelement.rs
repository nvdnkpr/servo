/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLBaseElementBinding;
use dom::bindings::utils::ErrorResult;
use dom::document::AbstractDocument;
use dom::element::HTMLBaseElementTypeId;
use dom::htmlelement::HTMLElement;
use dom::node::{AbstractNode, Node};
use servo_util::str::DOMString;

pub struct HTMLBaseElement {
    htmlelement: HTMLElement
}

impl HTMLBaseElement {
    pub fn new_inherited(localName: DOMString, document: AbstractDocument) -> HTMLBaseElement {
        HTMLBaseElement {
            htmlelement: HTMLElement::new_inherited(HTMLBaseElementTypeId, localName, document)
        }
    }

    pub fn new(localName: DOMString, document: AbstractDocument) -> AbstractNode {
        let element = HTMLBaseElement::new_inherited(localName, document);
        Node::reflect_node(@mut element, document, HTMLBaseElementBinding::Wrap)
    }
}

impl HTMLBaseElement {
    pub fn Href(&self) -> DOMString {
        ~""
    }

    pub fn SetHref(&self, _href: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Target(&self) -> DOMString {
        ~""
    }

    pub fn SetTarget(&self, _target: DOMString) -> ErrorResult {
        Ok(())
    }
}
