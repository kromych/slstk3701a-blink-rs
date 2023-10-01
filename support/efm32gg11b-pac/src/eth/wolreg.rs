#[doc = "Register `WOLREG` reader"]
pub type R = crate::R<WOLREG_SPEC>;
#[doc = "Register `WOLREG` writer"]
pub type W = crate::W<WOLREG_SPEC>;
#[doc = "Field `ADDR` reader - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `WOLMASK0` reader - Wake on LAN magic packet event enable"]
pub type WOLMASK0_R = crate::BitReader;
#[doc = "Field `WOLMASK0` writer - Wake on LAN magic packet event enable"]
pub type WOLMASK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WOLMASK1` reader - Wake on LAN ARP request event enable"]
pub type WOLMASK1_R = crate::BitReader;
#[doc = "Field `WOLMASK1` writer - Wake on LAN ARP request event enable"]
pub type WOLMASK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WOLMASK2` reader - Wake on LAN specific address register 1 event enable"]
pub type WOLMASK2_R = crate::BitReader;
#[doc = "Field `WOLMASK2` writer - Wake on LAN specific address register 1 event enable"]
pub type WOLMASK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WOLMASK3` reader - Wake on LAN multicast hash event enable"]
pub type WOLMASK3_R = crate::BitReader;
#[doc = "Field `WOLMASK3` writer - Wake on LAN multicast hash event enable"]
pub type WOLMASK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable"]
    #[inline(always)]
    pub fn wolmask0(&self) -> WOLMASK0_R {
        WOLMASK0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable"]
    #[inline(always)]
    pub fn wolmask1(&self) -> WOLMASK1_R {
        WOLMASK1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable"]
    #[inline(always)]
    pub fn wolmask2(&self) -> WOLMASK2_R {
        WOLMASK2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable"]
    #[inline(always)]
    pub fn wolmask3(&self) -> WOLMASK3_R {
        WOLMASK3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<WOLREG_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable"]
    #[inline(always)]
    #[must_use]
    pub fn wolmask0(&mut self) -> WOLMASK0_W<WOLREG_SPEC, 16> {
        WOLMASK0_W::new(self)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable"]
    #[inline(always)]
    #[must_use]
    pub fn wolmask1(&mut self) -> WOLMASK1_W<WOLREG_SPEC, 17> {
        WOLMASK1_W::new(self)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable"]
    #[inline(always)]
    #[must_use]
    pub fn wolmask2(&mut self) -> WOLMASK2_W<WOLREG_SPEC, 18> {
        WOLMASK2_W::new(self)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable"]
    #[inline(always)]
    #[must_use]
    pub fn wolmask3(&mut self) -> WOLMASK3_W<WOLREG_SPEC, 19> {
        WOLMASK3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Wake on LAN Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wolreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wolreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WOLREG_SPEC;
impl crate::RegisterSpec for WOLREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wolreg::R`](R) reader structure"]
impl crate::Readable for WOLREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wolreg::W`](W) writer structure"]
impl crate::Writable for WOLREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WOLREG to value 0"]
impl crate::Resettable for WOLREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
