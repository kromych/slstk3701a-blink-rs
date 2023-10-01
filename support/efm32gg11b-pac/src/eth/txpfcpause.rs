#[doc = "Register `TXPFCPAUSE` reader"]
pub type R = crate::R<TXPFCPAUSE_SPEC>;
#[doc = "Register `TXPFCPAUSE` writer"]
pub type W = crate::W<TXPFCPAUSE_SPEC>;
#[doc = "Field `VECTORENB` reader - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
pub type VECTORENB_R = crate::FieldReader;
#[doc = "Field `VECTORENB` writer - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
pub type VECTORENB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VECTOR` reader - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
pub type VECTOR_R = crate::FieldReader;
#[doc = "Field `VECTOR` writer - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
pub type VECTOR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub fn vectorenb(&self) -> VECTORENB_R {
        VECTORENB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub fn vector(&self) -> VECTOR_R {
        VECTOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn vectorenb(&mut self) -> VECTORENB_W<TXPFCPAUSE_SPEC, 0> {
        VECTORENB_W::new(self)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    #[must_use]
    pub fn vector(&mut self) -> VECTOR_W<TXPFCPAUSE_SPEC, 8> {
        VECTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit PFC Pause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txpfcpause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txpfcpause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXPFCPAUSE_SPEC;
impl crate::RegisterSpec for TXPFCPAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpfcpause::R`](R) reader structure"]
impl crate::Readable for TXPFCPAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txpfcpause::W`](W) writer structure"]
impl crate::Writable for TXPFCPAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPFCPAUSE to value 0"]
impl crate::Resettable for TXPFCPAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
